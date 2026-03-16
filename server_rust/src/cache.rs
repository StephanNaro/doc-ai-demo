// SPDX-License-Identifier: GPL-3.0-or-later

use anyhow::Context;
use lru::LruCache;
use once_cell::sync::Lazy;
use std::fs;
use std::num::NonZeroUsize;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

/// Global LRU cache for file contents (max 100 entries)
static FILE_CACHE: Lazy<Arc<Mutex<LruCache<PathBuf, String>>>> = Lazy::new(|| {
    Arc::new(Mutex::new(LruCache::new(NonZeroUsize::new(100).unwrap())))
});

/// Retrieve file content, loading from disk only once (or on cache miss).
pub fn get_cached_content(path: &Path) -> anyhow::Result<String> {
    let mut cache = FILE_CACHE.lock().unwrap();

    if let Some(cached) = cache.get(path) {
        return Ok(cached.clone());
    }

    let content = fs::read_to_string(path)
        .with_context(|| format!("Failed to read file: {}", path.display()))?;

    cache.put(path.to_path_buf(), content.clone());

    #[cfg(debug_assertions)]
    println!("Cached (LRU): {}", path.display());

    Ok(content)
}