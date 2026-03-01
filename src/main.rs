// SPDX-License-Identifier: GPL-3.0-or-later

use anyhow::{Context, Result};
use clap::Parser;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fs;
use std::path::Path;

#[derive(Parser, Debug)]
#[command(version, about = "Simple invoice AI demo using Ollama")]
struct Args {
    /// The question to ask about the invoices (e.g. "Extract total and due date from inv_001.txt")
    query: String,

    /// Optional: specific invoice filename (without .txt), e.g. inv_001
    #[arg(short, long)]
    file: Option<String>,

    /// Ollama model to use (default: llama3.2 or whatever you have)
    #[arg(short, long, default_value = "llama3.2")]
    model: String,
}

#[derive(Serialize)]
struct OllamaGenerateRequest {
    model: String,
    prompt: String,
    stream: bool,           // false = get full response at once
    options: Option<serde_json::Value>,
}

#[derive(Deserialize, Debug)]
struct OllamaGenerateResponse {
    response: String,
    done: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    // 1. Find and load invoice content
    let data_dir = Path::new("data/invoice-proc");
    let content = if let Some(filename) = args.file {
        // Specific file requested
        let path = data_dir.join(format!("{}.txt", filename));
        fs::read_to_string(&path).with_context(|| format!("Cannot read {}", path.display()))?
    } else {
        // For demo: load first file, or concatenate a few
        // (In real version: search by keyword in query)
        let path = data_dir.join("inv_001.txt");
        fs::read_to_string(&path).with_context(|| "No default file found")?
    };

    println!("Using invoice content from file:\n{}\n", content.lines().take(5).collect::<Vec<_>>().join("\n"));

    // 2. Build prompt
    let prompt = format!(
        r#"
You are an invoice extraction assistant. Answer concisely and accurately based ONLY on the provided invoice text.
Include the source invoice if relevant.
If the question is about extraction, return in this format:
- Vendor: ...
- Total Due: ...
- Due Date: ...
- Other key fields: ...

Invoice text:
{content}

Question: {query}

Answer:
"#,
    content = content,
    query = args.query,
);

    // 3. Call Ollama
    let client = Client::new();
    let ollama_url = "http://localhost:11434/api/generate";

    let request_body = OllamaGenerateRequest {
        model: args.model,
        prompt,
        stream: false,
        options: None,
    };

    let res = client
        .post(ollama_url)
        .json(&request_body)
        .send()
        .await
        .context("Failed to reach Ollama (is it running?)")?;

    if !res.status().is_success() {
        anyhow::bail!("Ollama returned error: {}", res.status());
    }

    let ollama_res: OllamaGenerateResponse = res.json().await.context("Failed to parse Ollama response")?;

    println!("\n=== AI Answer ===\n{}", ollama_res.response.trim());

    Ok(())
}