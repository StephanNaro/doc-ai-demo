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

5. Copy and paste `demo/forms.html` into a page accessible to your browser, eg an MVC, and open it in your browser and start querying!

## Usage Examples

Use the tabbed interface (which shows sample questions as placeholders) or send JSON via curl/Postman:

### Invoices
```bash
curl -X POST http://localhost:8001/query \
  -H "Content-Type: application/json" \
  -d '{"query": "What is the total due on INV-2025-001?", "category": "invoices"}'
```

### Employment Contracts
```bash
curl -X POST http://localhost:8001/query \
  -H "Content-Type: application/json" \
  -d '{"query": "What is Bob Smiths notice period?", "category": "contracts"}'
```

### Customer Support
```bash
curl -X POST http://localhost:8001/query \
  -H "Content-Type: application/json" \
  -d '{"query": "Summarize the damaged product complaint", "category": "support"}'
```

### Knowledge Base
```bash
curl -X POST http://localhost:8001/query \
  -H "Content-Type: application/json" \
  -d '{"query": "How many annual leave days for full-time?", "category": "knowledge"}'
```

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

## License

GPL 3.0

Built as a learning/hobby project inspired by real-world AI document tools.
