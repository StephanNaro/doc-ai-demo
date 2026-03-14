// SPDX-License-Identifier: GPL-3.0-or-later

pub mod ai;
pub use ai::query_ollama;

pub mod cache;
pub use cache::get_cached_content;

pub mod cla;
pub use cla::Args;
pub use clap::Parser;

pub mod data;
pub use data::{Category, ALL_CATEGORIES};

pub mod indexer;

pub mod retrieval;
pub use retrieval::find_relevant_files;

pub mod types;
pub use types::{ErrorResponse, ApiResponse, QueryRequest, Envelope};