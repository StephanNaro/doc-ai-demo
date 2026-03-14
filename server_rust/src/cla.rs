// SPDX-License-Identifier: GPL-3.0-or-later

// Command Line Arguments

use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name = "doc-ai-server",
    about = "Local AI-powered document Q&A server",
    version,
    author
)]

pub struct Args {
    /// Port to listen on
    #[arg(long, default_value_t = 8001)]
    pub port: u16,

    /// Ollama model name (e.g. llama3.2, phi3:mini)
    #[arg(long, default_value = "llama3.2")]
    pub model: String,
}