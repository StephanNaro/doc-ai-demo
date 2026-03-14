// SPDX-License-Identifier: GPL-3.0-or-later

use std::collections::HashSet;
use std::path::{Path, PathBuf};

use crate::Category;
use crate::indexer::INVERTED_INDEX;

pub fn find_relevant_files(query: &str, category: &Category) -> Vec<PathBuf> {
    let base_dir = category.folder_path();

    let lower_query = query.to_lowercase();
    let query_words: HashSet<String> = lower_query
        .split_whitespace()
        .filter(|w| w.len() > 2)
        .map(|w| w.to_string())
        .collect();

    if query_words.is_empty() {
        return vec![];
    }

    // Collect candidates + count matches
    let mut scored_files: Vec<(String, usize)> = Vec::new();

    for (word, filenames) in INVERTED_INDEX.iter() {
        if query_words.contains(word) {
            for fname in filenames {
                // Find or insert with score
                if let Some((_, score)) = scored_files.iter_mut().find(|(f, _)| f == fname) {
                    *score += 1;
                } else {
                    scored_files.push((fname.clone(), 1));
                }
            }
        }
    }

    // Sort: highest score first, then stable by filename
    scored_files.sort_by(|a, b| {
        b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0))
    });

    // Take top N
    let max_results = 4;
    let top_fnames: Vec<String> = scored_files
        .into_iter()
        .take(max_results)
        .map(|(f, score)| {
            println!("Selected: {} (score: {})", f, score); // debug
            f
        })
        .collect();

    top_fnames
        .into_iter()
        .filter_map(|fname| {
            let path = Path::new(base_dir).join(&fname);
            path.exists().then_some(path.to_path_buf())
        })
        .collect()
}