# Algorithms & Data Structures in Doc API Demo

## Overview

This project started as a simple local AI-powered document Q&A tool but quickly became an opportunity to apply real algorithms and data structures in a practical context.

The goal was never to build the most advanced search engine, but to:
- Demonstrate core CS concepts in a working system
- Show understanding of trade-offs (time vs space, simplicity vs scalability)

Many of the techniques used (or planned) are foundational ideas from search engines, information retrieval, caching systems, and RAG (Retrieval-Augmented Generation) pipelines.

The list below covers both what is already implemented and what could be added in future iterations.

## Already Incorporated

These DS&A concepts are actively used in the current codebase.

1. **Inverted Index**  
   - **Used in**: `indexer.rs` → `INVERTED_INDEX`  
   - **Purpose**: Fast keyword → document lookup (O(1) average-case per word)  
   - **Implementation**: `HashMap<String, Vec<String>>` built at startup  
   - **Benefit**: Avoids scanning every file on each query  
   - **Current limitation**: Exact word match only (no stemming/lemmatization/synonyms)

2. **Memoization / Content Caching**  
   - **Used in**: `cache.rs` → `FILE_CACHE` (LRU via `lru` crate)  
   - **Purpose**: Avoid repeated disk I/O — files loaded only once  
   - **Implementation**: Global `LruCache<PathBuf, String>` (max 100 entries)  
   - **Benefit**: Space-for-time tradeoff; essential when same files are queried repeatedly

3. **Relevance Scoring (simple term overlap)**  
   - **Used in**: `relevance.rs` → `find_relevant_files`  
   - **Purpose**: Rank documents by how many query words they contain  
   - **Implementation**: Count unique matching words → sort descending → take top 4  
   - **Benefit**: Better than arbitrary selection; still very lightweight

4. **HashMap / HashSet**  
   - **Used throughout**: inverted index, scoring, deduplication of filenames  
   - **Purpose**: O(1) lookups, fast membership testing, deduplication

5. **Lazy Initialization**  
   - **Used in**: `once_cell::sync::Lazy` for index and cache  
   - **Purpose**: Build expensive structures only on first use (not at compile time)

6. **Mutex + Arc for thread-safety**  
   - **Used in**: global cache and index  
   - **Purpose**: Safe concurrent access in Rocket's multi-threaded runtime

## Unrealised / Future Ideas

These are concepts that could be added to make the system more sophisticated.  
Most are realistic next steps for scaling beyond a demo.

1. **Top-k selection with Priority Queue / BinaryHeap**  
   - **Status**: Not implemented  
   - **Potential**: Replace `Vec` sort with `BinaryHeap` for efficient top-N extraction  
   - **Complexity**: O(n log k) vs current O(n log n)  
   - **Benefit**: Faster when n is large

2. **TF-IDF scoring**  
   - **Status**: Not implemented  
   - **Potential**: Weight rare words higher than common ones  
   - **Benefit**: Improves relevance over raw term count

3. **Dense vector / semantic search (embeddings + cosine similarity)**  
   - **Status**: Planned but not implemented  
   - **Potential**: Replace keyword matching with sentence-transformers or Ollama embeddings  
   - **Future**: Use `hnsw` or `qdrant-lite` for approximate nearest neighbors  
   - **Benefit**: Handles synonyms, paraphrasing ("notice period" ≈ "termination notice")

4. **Trie / Prefix tree**  
   - **Status**: Not implemented  
   - **Potential**: For query auto-completion or prefix-based filtering  
   - **Benefit**: O(m) lookup where m = prefix length

5. **Aho-Corasick (multi-pattern string matching)**  
   - **Status**: Not implemented  
   - **Potential**: Faster multi-keyword search in documents  
   - **Benefit**: O(n + z) vs naive `.contains()`

6. **Graph traversal (BFS/DFS)**  
   - **Status**: Not implemented  
   - **Potential**: For cross-document linking (e.g. "find related contracts")  
   - **Benefit**: Answer comparative or entity-based questions

7. **Dynamic programming / memoization of LLM responses**  
   - **Status**: Not implemented  
   - **Potential**: Cache identical query+category+files combos  
   - **Benefit**: Avoid redundant Ollama calls

8. **Background incremental indexing**  
   - **Status**: Not implemented  
   - **Potential**: Use `notify` crate to watch folders and rebuild index parts on file change  
   - **Benefit**: Handle real, changing document collections

9. **Sharding / distributed index**  
   - **Status**: Far future  
   - **Potential**: Split index across multiple machines or processes  
   - **Benefit**: Scale to millions of documents

10. **LRU / LFU cache eviction policy**  
    - **Status**: Partially implemented (LRU in `cache.rs`)  
    - **Future**: Add bounded cache size for index entries themselves

## Comments & Reflections

- The current keyword-based inverted index is crude but fast and deterministic.
- It misses synonyms/variants, but Ollama's language understanding compensates somewhat.
- For a demo this is acceptable; in production we'd move to embeddings + vector search.
- Caching + LRU already prevent most I/O bottlenecks for small-to-medium collections.
- The project shows that even simple DS&A (HashMap, sorting, caching) can create a usable system — no need for over-engineering at demo scale.

Future directions would likely involve:
- Moving from keyword → dense vector retrieval
- Adding incremental updates instead of full re-index on startup
- Implementing proper ranking (BM25 or learned model)