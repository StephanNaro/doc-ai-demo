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
) -> Result<String> {
    let client = Client::new();
    let ollama_url = "http://localhost:11434/api/generate";

    let prompt = format!(
        r#"You are a precise invoice processor. Answer using ONLY the provided data.
Be concise. Cite sources (file names) when possible.

For extraction/summary questions return JSON like:
{{
  "answer": "brief summary or extracted value",
  "sources": ["inv_001.txt", ...],
  "details": {{ ... optional fields ... }}
}}

Documents:
{contents}

Question: {query}

Respond with JSON only."#,
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