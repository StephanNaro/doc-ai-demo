// SPDX-License-Identifier: GPL-3.0-or-later

use anyhow::{Context, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::Category;

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
    //pub done: bool,
}

pub async fn query_ollama(
    model: &str,
    contents: String,
    query: &str,
    category: &Category,
) -> Result<String> {
    let client = Client::new();
    let ollama_url = "http://localhost:11434/api/generate";

    let system_role = category.ai_instruction();

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