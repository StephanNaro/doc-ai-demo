// SPDX-License-Identifier: GPL-3.0-or-later

use anyhow::Context;
use lru::LruCache;
use once_cell::sync::Lazy;
use std::fs;
use std::num::NonZeroUsize;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

// LRU cache with max 100 files (adjust as needed)
static FILE_CACHE: Lazy<Arc<Mutex<LruCache<PathBuf, String>>>> = Lazy::new(|| {
    Arc::new(Mutex::new(LruCache::new(NonZeroUsize::new(100).unwrap())))
});

pub fn get_cached_content(path: &Path) -> anyhow::Result<String> {
    let mut cache = FILE_CACHE.lock().unwrap();

    if let Some(cached) = cache.get(path) {
        return Ok(cached.clone());
    }

    let content = fs::read_to_string(path)
        .with_context(|| format!("Failed to read {}", path.display()))?;

    cache.put(path.to_path_buf(), content.clone());
    println!("Cached (LRU): {}", path.display());

    Ok(content)
}