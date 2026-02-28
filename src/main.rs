// SPDX-License-Identifier: GPL-3.0-or-later

use anyhow::{Context, Result};
use clap::Parser;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Parser, Debug)]
#[command(version, about = "Invoice AI demo using Ollama - scans folder for relevant files")]
struct Args {
    /// The question to ask about the invoices
    query: String,

    /// Ollama model to use (e.g. llama3.2)
    #[arg(short, long, default_value = "llama3.2")]
    model: String,
}

#[derive(Serialize)]
struct OllamaRequest {
    model: String,
    prompt: String,
    stream: bool,
    format: String,           // "json" to force structured output
    options: Option<serde_json::Value>,
}

#[derive(Deserialize, Debug)]
struct OllamaResponse {
    response: String,
    done: bool,
}

fn find_relevant_files(data_dir: &Path, query: &str) -> Vec<PathBuf> {
    let mut matches = Vec::new();
    let lower_query = query.to_lowercase();

    if let Ok(entries) = fs::read_dir(data_dir) {
        for entry in entries.flatten() {
            if let Some(file_name) = entry.file_name().to_str() {
                if file_name.ends_with(".txt") {
                    let path = entry.path();
                    // Simple heuristic: if query mentions invoice number or file stem
                    let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("").to_lowercase();
                    if lower_query.contains(&stem) || lower_query.contains("invoice") {
                        matches.push(path);
                    }
                }
            }
        }
    }

    // Fallback: if no specific match, use all files (or first few)
    if matches.is_empty() {
        if let Ok(entries) = fs::read_dir(data_dir) {
            for entry in entries.flatten().take(1) {
                if entry.path().extension().and_then(|e| e.to_str()) == Some("txt") {
                    matches.push(entry.path());
                }
            }
        }
    }

    matches
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let data_dir = Path::new("data/invoice-proc");
    if !data_dir.exists() {
        anyhow::bail!("Data folder not found: {}", data_dir.display());
    }

    println!("Scanning folder: {}", data_dir.display());

    // 1. Find relevant invoice files
    let relevant_files = find_relevant_files(data_dir, &args.query);
    if relevant_files.is_empty() {
        anyhow::bail!("No invoice files found in {}", data_dir.display());
    }

    println!("Using {} relevant invoice(s):", relevant_files.len());
    for path in &relevant_files {
        println!("  - {}", path.display());
    }

    // 2. Load content from files
    let mut contents = String::new();
    for path in relevant_files {
        let text = fs::read_to_string(&path)
            .with_context(|| format!("Failed to read {}", path.display()))?;
        contents.push_str(&format!("\n--- Invoice from {} ---\n{}\n", path.display(), text));
    }

    // 3. Build structured prompt
    let prompt = format!(
        r#"You are a precise invoice calculator. Use ONLY the numbers and text below. Do NOT invent values.

Rules:
- Return ONLY valid JSON — no explanations outside the JSON.
- For sums: add exactly the totals shown; do not round or estimate.
- List every invoice you used.
- If asked for summary of amounts, output: {{ "total_sum": "Rxx,xxx.xx", "invoices": [{{ "file": "...", "total": "R..." }}, ...] }}

Documents:
{contents}

Question: {query}

Respond with JSON only."#,
        contents = contents,
        query = args.query,
    );

    // 4. Call Ollama
    let client = Client::new();
    let ollama_url = "http://localhost:11434/api/generate";

    let request_body = OllamaRequest {
        model: args.model,
        prompt,
        stream: false,
        format: "json".to_string(),
        options: Some(json!({
            "temperature": 0.0,
            "top_p": 0.95,
        })),
    };

    println!("\nSending request to Ollama... (this may take 5-30 seconds with llama3.2)");

    let res = client
        .post(ollama_url)
        .json(&request_body)
        .send()
        .await
        .context("Cannot reach Ollama — is `ollama serve` or `ollama run llama3.2` active?")?;

    let status = res.status();

    if !status.is_success() {
        let text = res.text().await.unwrap_or_else(|_| "No response body".to_string());
        anyhow::bail!(
            "Ollama failed with status {}.\nBody: {}",
            status,
            text.trim()
        );
    }

    let ollama_res: OllamaResponse = res.json().await.context("Invalid JSON from Ollama")?;

    println!("\n=== Structured Answer (JSON) ===\n{}", ollama_res.response.trim());

    // Optional pretty-print
    if let Ok(json_val) = serde_json::from_str::<serde_json::Value>(&ollama_res.response) {
        println!("\nPretty-printed:\n{}", serde_json::to_string_pretty(&json_val)?);
    }

    Ok(())
}