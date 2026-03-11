// SPDX-License-Identifier: GPL-3.0-or-later

use anyhow::{Context, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::fs;
use std::path::{Path, PathBuf};

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

pub fn find_relevant_files(data_dir: &Path, query: &str) -> Vec<PathBuf> {
    let mut matches = Vec::new();
    let lower_query = query.to_lowercase();

    if let Ok(entries) = fs::read_dir(data_dir) {
        for entry in entries.flatten() {
            if let Some(file_name) = entry.file_name().to_str() {
                if file_name.ends_with(".txt") {
                    let path = entry.path();
                    let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("").to_lowercase();
                    if lower_query.contains(&stem) || lower_query.contains("invoice") {
                        matches.push(path);
                    }
                }
            }
        }
    }

    if matches.is_empty() {
        if let Ok(entries) = fs::read_dir(data_dir) {
            for entry in entries.flatten().take(2) {  // keep small
                if entry.path().extension().and_then(|e| e.to_str()) == Some("txt") {
                    matches.push(entry.path());
                }
            }
        }
    }

    matches
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