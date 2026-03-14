// SPDX-License-Identifier: GPL-3.0-or-later

#[macro_use]
extern crate rocket;

use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::{Header, Status};
use rocket::Request;
use rocket::Response;   // Somehow different from response...
use rocket::response::{self, Responder};
use rocket::serde::json::Json;
use rocket::State;
use serde_json::{json, Value};
use std::sync::Arc;

use doc_ai_server::*;

// CORS fairing
struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _req: &'r Request<'_>, res: &mut Response<'r>) {
        res.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        res.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, OPTIONS"));
        res.set_header(Header::new("Access-Control-Allow-Headers", "Content-Type"));
    }
}

// CORS wrapper
struct CorsResponder<R>(R);

impl<'r, 'o: 'r, R: Responder<'r, 'o>> Responder<'r, 'o> for CorsResponder<R> {
    fn respond_to(self, request: &'r Request<'_>) -> response::Result<'o> {
        let mut res = self.0.respond_to(request)?;
        res.set_header(Header::new("Access-Control-Allow-Origin", "*")); // or specific origin like "http://localhost:your-mvc-port"
        res.set_header(Header::new("Access-Control-Allow-Methods", "POST, OPTIONS"));
        res.set_header(Header::new("Access-Control-Allow-Headers", "Content-Type"));
        Ok(res)
    }
}

// OPTIONS preflight
#[options("/query")]
fn options_handler() -> CorsResponder<Status> {
    CorsResponder(Status::Ok)
}

// Main query handler
#[post("/query", format = "json", data = "<req>")]
async fn query(
    req: Json<QueryRequest>,
    state: &State<Arc<Args>>,
) -> CorsResponder<Json<Value>> {
    let category_str = req.category.as_deref().unwrap_or_default();
    let category = match Category::from_api_value(category_str) {
        Some(cat) => cat,
        None => {
            let err = ErrorResponse {
                error: true,
                code: "invalid_category".to_string(),
                message: format!(
                    "Unknown category '{}'. Valid values: {}",
                    category_str,
                    Category::all_api_values_human()
                ),
                category: Some(category_str.to_string()),
                query: Some(req.query.clone()),
            };
            return CorsResponder(Envelope::failure(err).into());
        }
    };

    let relevant_files = find_relevant_files(&req.query, &category);

    if relevant_files.is_empty() {
        let err = ErrorResponse {
            error: true,
            code: "no_matches".to_string(),
            message: format!("No relevant documents found in '{}' category", category.display_name()),
            category: Some(category.api_value().to_string()),
            query: Some(req.query.clone()),
        };
        return CorsResponder(Envelope::failure(err).into());
    }

    let mut contents = String::new();
    let mut file_names = Vec::new();

    for path in relevant_files {
        let text = match get_cached_content(&path) {
            Ok(t) => t,
            Err(e) => {
                let err = ErrorResponse {
                    error: true,
                    code: "internal_server_error".to_string(),
                    message: e.to_string(),
                    category: Some(category.api_value().to_string()),
                    query: Some(req.query.clone()),
                };
                return CorsResponder(Envelope::failure(err).into());
            }
        };

        let fname = path.file_name().unwrap().to_string_lossy().to_string();
        contents.push_str(&format!("\n--- {} ---\n{}\n", fname, text));
        file_names.push(fname);
    }

    match query_ollama(&state.model.clone(), contents, &req.query, &category).await {
        Ok(raw_json) => {
            let parsed: Value = serde_json::from_str(&raw_json)
                .unwrap_or_else(|_| json!({"raw": raw_json}));

            let api_resp = ApiResponse {
                answer: parsed,
                used_files: file_names,
                error: None,
            };

            CorsResponder(Envelope::success(api_resp).into())
        }
        Err(e) => {
            let err = ErrorResponse {
                error: true,
                code: "ollama_error".to_string(),
                message: e.to_string(),
                category: Some(category.api_value().to_string()),
                query: Some(req.query.clone()),
            };
            CorsResponder(Envelope::failure(err).into())
        }
    }
}

// Startup validation
#[launch]
fn rocket() -> _ {
    let config = Args::parse();

    // Validate folders
    for cat in ALL_CATEGORIES {
        let path = std::path::Path::new(cat.folder_path());
        if !path.exists() || !path.is_dir() {
            eprintln!("ERROR: Required data folder missing: {}", cat.folder_path());
            std::process::exit(1);
        }
    }

    println!("All data folders found. Starting server on port {}", config.port);
    println!("Using Ollama model: {}", config.model);
    println!("Supported categories:");
    for cat in ALL_CATEGORIES {
        println!("- {} ({}) → {}", cat.display_name(), cat.api_value(), cat.folder_path());
    }

    rocket::build()
        .configure(rocket::Config::figment().merge(("port", config.port)))
        .attach(CORS)
        .mount("/", routes![query, options_handler])
        .manage(Arc::new(config))
}