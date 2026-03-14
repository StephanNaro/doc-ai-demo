// SPDX-License-Identifier: GPL-3.0-or-later

use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;

use crate::ALL_CATEGORIES;
use crate::get_cached_content;

// Uses cache
pub static INVERTED_INDEX: Lazy<HashMap<String, Vec<String>>> = Lazy::new(|| {
    let mut index: HashMap<String, Vec<String>> = HashMap::new();
    let word_re = Regex::new(r"\b\w+\b").unwrap();

    for category in ALL_CATEGORIES {
        let dir = Path::new(category.folder_path());
        if !dir.exists() { continue; }

        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().and_then(|e| e.to_str()) == Some("txt") {
                    // ← Use the cache here (so files are loaded only once)
                    if let Ok(text) = get_cached_content(&path) {
                        let lower_text = text.to_lowercase();
                        for cap in word_re.captures_iter(&lower_text) {
                            if let Some(m) = cap.get(0) {
                                let word = m.as_str().to_string();
                                let fname = path.file_name().unwrap().to_string_lossy().to_string();
                                index.entry(word).or_default().push(fname);
                            }
                        }
                    }
                }
            }
        }
    }

    // Deduplicate filename lists
    for files in index.values_mut() {
        let set: HashSet<_> = files.drain(..).collect();
        *files = set.into_iter().collect();
    }

    println!("✅ Inverted index built with {} unique words. All files cached.", index.len());
    index
});