## Algorithms & Data Structures Used

### Retrieval with Inverted Index

Instead of scanning every file on each query, I build a simple inverted index at startup:
- HashMap<String, Vec<String>> where key = normalized word, value = filenames containing it
- Query words → union of matching files → score by overlap → top N fed to LLM

This gives O(1) average-case word lookup and scales better than linear scan.

“This is the core idea behind how search engines work.”

### File Content Caching

To avoid repeated disk reads, file contents are cached in a thread-safe `HashMap<PathBuf, String>` using `once_cell::sync::Lazy` + `Mutex`.  
First access loads from disk; all later requests hit memory.

This is a simple but effective **memoization** pattern (space-for-time tradeoff) — common when the same documents are queried repeatedly.

### LRU Cache, Etc.

Grok and I have done further work, but I think I'll be switching to C# in stead of Rust.










## Grok's First Overview - Delete As Implemented
Your current demo is already using several DS&A ideas implicitly, but we can make them much more explicit (both in code and in how you talk about the project). Below is a realistic list of places where classic algorithms & data structures either are already present or could be meaningfully added/improved — ranked roughly by how impressive/easy they are to implement and explain in an interview context.

| # | Concept / Data Structure | Where it fits in your demo | Current state | Suggested improvement / explicit version |
|---|---------------------------|----------------------------|---------------|-------------------------------------------|
| 3 | **Priority Queue / Top-K selection** | Selecting top-N most relevant documents | Currently take(2) or keyword contains | Use `BinaryHeap` (max-heap) or `std::collections::BinaryHeap` with a custom score (e.g. number of matching keywords, or later tf-idf) |
| 4 | **Trie / Prefix tree** | If you ever add autocomplete for queries | Not present | Optional future: Trie of all words in documents for query auto-complete |
| 5 | **Vector similarity search** (cosine similarity + approximate nearest neighbors) | Semantic retrieval (the “real” RAG upgrade) | Keyword only | Replace keyword matching with sentence-transformers embeddings → store vectors in Vec<Vec<f32>> or use a crate like `hnsw` / `qdrant-lite` / `lance` for ANN search |
| 6 | **Sliding window / chunking algorithm** | Document splitting (currently whole files) | Whole-file content | Implement paragraph/sentence-aware chunking + overlapping windows |
| 7 | **Sorting + stable sort** | Ranking results by score | No ranking | After scoring chunks/documents, sort by descending score (`.sort_by(|a,b| b.score.cmp(&a.score))`) |
| 8 | **String searching algorithms** (Boyer-Moore, KMP, or regex) | Current keyword matching | Naive `.contains()` | Upgrade to `aho-corasick` crate (multi-pattern Aho-Corasick automaton) for fast multi-keyword search |
| 9 | **Graph traversal** (if you add cross-document linking) | Future: “find related contracts” | Not present | Model documents as nodes, shared entities (names, dates) as edges → BFS/DFS for related docs |
| 10 | **Dynamic programming / memoization** | Prompt caching / repeated queries | Not present | Memoize Ollama responses for identical query+category+files combo using `HashMap<String, String>` |

### Quick wins you can do in < 1 hour that look strong in interviews

1. **Inverted index** (most bang-for-buck)
   - Completed

2. **Top-k selection with BinaryHeap**
   - Score each file (e.g. count of matching keywords)
   - Use `BinaryHeap` to keep only the top 3–5

3. **Simple caching**
   - Completed
