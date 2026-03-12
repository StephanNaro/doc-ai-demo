// SPDX-License-Identifier: GPL-3.0-or-later

use anyhow::{Context, Result};
use lru::LruCache;
use once_cell::sync::Lazy;
use regex::Regex;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::num::NonZeroUsize;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

#[derive(Serialize)]
pub struct OllamaRequest {
    pub model: String,
    pub prompt: String,
    pub stream: bool,
    pub format: String,
    pub options: Option<Value>,
}

#[derive(Deserialize, Debug)]
pub struct OllamaResponse {
    pub response: String,
    pub done: bool,
}

// ==================== GLOBAL CACHE ====================
// LRU cache with max 100 files (adjust as needed)
static FILE_CACHE: Lazy<Arc<Mutex<LruCache<PathBuf, String>>>> = Lazy::new(|| {
    Arc::new(Mutex::new(LruCache::new(NonZeroUsize::new(100).unwrap())))
});

pub fn get_cached_content(path: &Path) -> anyhow::Result<String> {
    let mut cache = FILE_CACHE.lock().unwrap();

    if let Some(cached) = cache.get(path) {
        return Ok(cached.clone());
    }

    let content = fs::read_to_string(path)
        .with_context(|| format!("Failed to read {}", path.display()))?;

    cache.put(path.to_path_buf(), content.clone());
    println!("Cached (LRU): {}", path.display());

    Ok(content)
}

// ==================== INVERTED INDEX (uses cache) ====================
static INVERTED_INDEX: Lazy<HashMap<String, Vec<String>>> = Lazy::new(|| {
    let mut index: HashMap<String, Vec<String>> = HashMap::new();
    let word_re = Regex::new(r"\b\w+\b").unwrap();

    let categories = vec![
        ("data/invoices", "invoices"),
        ("data/employment-contracts", "contracts"),
        ("data/customer-support", "support"),
        ("data/knowledge-base", "knowledge"),
    ];

    for (dir_str, _) in categories {
        let dir = Path::new(dir_str);
        if !dir.exists() { continue; }

        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().and_then(|e| e.to_str()) == Some("txt") {
                    // ← Use the cache here (so files are loaded only once)
                    if let Ok(text) = get_cached_content(&path) {
                        let lower_text = text.to_lowercase();
                        for cap in word_re.captures_iter(&lower_text) {
                            if let Some(m) = cap.get(0) {
                                let word = m.as_str().to_string();
                                let fname = path.file_name().unwrap().to_string_lossy().to_string();
                                index.entry(word).or_default().push(fname);
                            }
                        }
                    }
                }
            }
        }
    }

    // Deduplicate filename lists
    for files in index.values_mut() {
        let set: HashSet<_> = files.drain(..).collect();
        *files = set.into_iter().collect();
    }

    println!("✅ Inverted index built with {} unique words. All files cached.", index.len());
    index
});

pub fn find_relevant_files(query: &str, category: &str) -> Vec<PathBuf> {
    let lower_query = query.to_lowercase();
    let query_words: HashSet<String> = lower_query
        .split_whitespace()
        .filter(|w| w.len() > 2)
        .map(|w| w.to_string())
        .collect();

    if query_words.is_empty() {
        return vec![];
    }

    // Collect candidates + count matches
    let mut scored_files: Vec<(String, usize)> = Vec::new();

    for (word, filenames) in INVERTED_INDEX.iter() {
        if query_words.contains(word) {
            for fname in filenames {
                // Find or insert with score
                if let Some((_, score)) = scored_files.iter_mut().find(|(f, _)| f == fname) {
                    *score += 1;
                } else {
                    scored_files.push((fname.clone(), 1));
                }
            }
        }
    }

    // Sort: highest score first, then stable by filename
    scored_files.sort_by(|a, b| {
        b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0))
    });

    // Take top N
    let max_results = 4;
    let top_fnames: Vec<String> = scored_files
        .into_iter()
        .take(max_results)
        .map(|(f, score)| {
            println!("Selected: {} (score: {})", f, score); // debug
            f
        })
        .collect();

    // Map to full paths
    let base_dir = match category {
        "contracts" | "employment-contracts" => "data/employment-contracts",
        "support"   => "data/customer-support",
        "knowledge" => "data/knowledge-base",
        _           => "data/invoices",
    };

    top_fnames
        .into_iter()
        .filter_map(|fname| {
            let path = Path::new(base_dir).join(&fname);
            path.exists().then_some(path.to_path_buf())
        })
        .collect()
}

pub async fn query_ollama(
    model: &str,
    contents: String,
    query: &str,
    category: &str,
) -> Result<String> {
    let client = Client::new();
    let ollama_url = "http://localhost:11434/api/generate";

    let system_role = match category {
    "contracts" | "employment-contracts" | "contract" | "employment" => 
        "You are an expert employment contract reviewer. Focus on clauses, notice periods, leave entitlement, salary, non-compete, confidentiality, probation, remote work, etc. Cite exact wording where possible.",

    "support" | "customer-support" | "tickets" | "support-tickets" => 
        "You are a customer support analyst. Summarize the issue, customer details, requested action, severity, and suggest a helpful next reply or resolution steps.",

    "knowledge" | "knowledge-base" | "kb" | "policies" | "faq" => 
        "You are a company policy and internal knowledge assistant. Answer clearly and directly from the provided documents. Quote sections or rules verbatim when relevant.",

    _ =>  // defaults to invoices
        "You are a precise invoice extraction and summarization assistant. Extract vendor, amounts (subtotal, VAT, total due), due date, invoice number, and payment terms exactly as written. Perform simple sums only if explicitly asked.",
};
    
    let prompt = format!(
        r#"{system_role}

Rules:
- Answer using ONLY the provided documents.
- Return ONLY valid JSON — no extra text outside the JSON object.
- Always include a "sources" array with the file names used.
- Use appropriate keys depending on the category (e.g. "total_due", "notice_period", "issue_summary", "policy_answer").

Documents:
{contents}

Question: {query}

Respond with JSON only."#,
        system_role = system_role,
        contents = contents,
        query = query,
    );

    let request_body = OllamaRequest {
        model: model.to_string(),
        prompt,
        stream: false,
        format: "json".to_string(),
        options: Some(json!({
            "temperature": 0.0,
            "top_p": 0.95,
        })),
    };

    let res = client
        .post(ollama_url)
        .json(&request_body)
        .send()
        .await
        .context("Cannot reach Ollama")?;

    let status = res.status();
    if !status.is_success() {
        let text = res.text().await.unwrap_or_default();
        anyhow::bail!("Ollama error {}: {}", status, text);
    }

    let ollama_res: OllamaResponse = res.json().await.context("Invalid Ollama response")?;
    Ok(ollama_res.response)
}