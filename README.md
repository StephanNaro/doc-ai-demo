# Doc API Demo – Local AI-Powered Document Q&A

A simple, fully local, privacy-focused demo that lets you chat with your documents using a lightweight LLM (Ollama + llama3.2) running on your laptop.

It supports four document categories:
- Invoices (extraction of amounts, dates, vendors)
- Employment Contracts (clause lookup, notice periods, leave)
- Customer Support Tickets (summarization, issue extraction)
- Knowledge Base / Policies (FAQ-style answers)

Built with Rust (Rocket backend) + local C# & plain HTML/JS frontends. No cloud APIs, no data leaves your machine.

## Features

- REST API endpoint `/query` accepting natural-language questions
- Rocket's port and the model used by Ollama are configurable via the command line
- Category-aware prompting (different system roles per document type)
- Pretty-printed JSON responses with source file references
- CORS support for cross-origin requests
- C# desktop client (WinForms) for native feel
- HTML demo shows tabbed interface for easy switching between document types

## Algorithms & Data Structures

See [DSAideas.md](./DSAideas.md) for a detailed overview of DS&A concepts used in or suggested for this project.

## Requirements

- Rust (stable toolchain) – install from https://rustup.rs
- Ollama – download from https://ollama.com
- llama3.2 model (or any model you prefer for your hardware)
  ```bash
  ollama pull llama3.2
  ```
- .NET 10.0 (C#) - install from https://dotnet.microsoft.com/en-us/download

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
   → Listens on http://localhost:8001

5. Build desktop frontend:
   ```bash
   cd DocAiClient
   dotnet build
   ```

## Usage Examples

### Command Line
   See the test-script in `test/curl-test.sh` for `curl` usage examples.

### HTML
   `html_demo/tabbed.html` contains a tabbed interface showing sample questions as placeholders and allowing interactive querying. The file can be loaded directly into your browser for demo purposes, but is best wrapped in suitable HTML, PHP, etc.

### Desktop Application
   ```bash
   dotnet run
   ```
   Choose which document category to query with the drop-down menu. Enter a question relevant to the category of documents, click the **Ask** button, and wait for a response (which may take 30 seconds on a slow machine like mine). For examples of questions please see `test/curl-test.sh` or `html_demo/tabbed.html`.

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

- Add semantic search / embeddings for better file relevance.
- The desktop frontend app could be polished to be as user-friendly as the HTML demo. Or better, eg implement multi-turn chat (keep history in prompt or session).
- Better error handling & loading states.

## License, Inspiration, Development, and Disclaimer

This project is licensed under the GNU General Public License v3.0 (GPL-3.0). See [LICENSE](LICENSE) for details.

Built as a learning/hobby project inspired by a job ad and real-world AI document tools, with considerable assistance from an AI agent.

It should be noted that **no thought has been given to security** in this project, as my knowledge of security matters is very limited.