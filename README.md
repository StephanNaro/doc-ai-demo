# Doc API Demo – Local AI-Powered Document Q&A

A simple, fully local, privacy-focused demo that lets you chat with your documents using a lightweight LLM (Ollama + llama3.2) running on your laptop.

It supports four document categories:
- Invoices (extraction of amounts, dates, vendors)
- Employment Contracts (clause lookup, notice periods, leave)
- Customer Support Tickets (summarization, issue extraction)
- Knowledge Base / Policies (FAQ-style answers)

Built with Rust (Rocket backend) + plain HTML/JS frontend. No cloud APIs, no data leaves your machine.

## Features

- REST API endpoint `/query` accepting natural-language questions
- Category-aware prompting (different system roles per document type)
- HTML shows tabbed interface for easy switching between document types
- Pretty-printed JSON responses with source file references
- CORS support for cross-origin requests

## Requirements

- Rust (stable toolchain) – install from https://rustup.rs
- Ollama – download from https://ollama.com
- llama3.2 model (or any model you prefer for your hardware)
  ```bash
  ollama pull llama3.2
  ```

## Setup

1. Clone the repo:
   ```bash
   git clone https://github.com/StephanNaro/doc-ai-demo.git
   cd doc-api-demo
   ```

2. Install dependencies:
   ```bash
   cargo build
   ```

3. Start Ollama in a separate terminal:
   ```bash
   ollama run llama3.2
   ```

4. Run the server:
   ```bash
   cargo run
   ```
   → Listens on http://localhost:8001 (configurable via code or env `ROCKET_PORT`)

## Usage Examples

### Command Line
See, or run, the test-script in `test/curl-test.sh` for `curl` usage examples.

### HTML
`demo/tabbed.html` contains a tabbed interface showing sample questions as placeholders and allowing interactive querying. The file can be loaded directly into your browser for demo purposes, but is best wrapped in suitable HTML, PHP, etc.

## Known Limitations

- Using **llama3.2** (small model for low-end laptops) → answers can be **erratic**, inconsistent across runs, or contain small hallucinations/math errors.
  - For instance: Arithmetic is unreliable in tiny models - requesting a summarized total of invoices may list the correct values, but produce an incorrect total.
  - Better results should result with larger models (e.g. llama3.1:8b, phi3:mini) if your hardware allows. (Mine doesn't.)
  - Temperature fixed at 0.0 for determinism, but still not perfect.
  - Please note that these issues fall outside the scope of this demo.
- File relevance uses simple keyword matching → no semantic search/embeddings yet.
- No chat history / multi-turn conversation (single query only).
- Demo data is fake/static (in `data/` folders) — real use would index your own PDFs/docs.

## Possible Future Improvements

- C# desktop client (WinForms/WPF) for native feel
- Configure Rocket's port via command line
- Add semantic search / embeddings for better file relevance
- Multi-turn chat (keep history in prompt or session)
- Better error handling & loading states

## License, Inspiration, and Development

This project is licensed under the GNU General Public License v3.0 (GPL-3.0). See [LICENSE](LICENSE) for details.

Built as a learning/hobby project inspired by a job ad and real-world AI document tools, with considerable assistance from an AI agent.
