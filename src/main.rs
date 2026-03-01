// SPDX-License-Identifier: GPL-3.0-or-later

#[macro_use]
extern crate rocket;

use anyhow::Result;
use rocket::http::{Header, Status};
use rocket::Request;
use rocket::Response;
use rocket::response::{status, self, Responder};
use rocket::serde::json::Json;
use rocket::{State, fs::relative};
use rocket::fairing::{Fairing, Info, Kind};
use std::sync::Arc;

use doc_ai_demo::*;

#[derive(serde::Deserialize)]
struct QueryRequest {
    query: String,
    #[serde(default)]  // makes category optional, defaults to None
    category: Option<String>,
}

#[derive(serde::Serialize)]
struct ApiResponse {
    answer: serde_json::Value,  // the raw JSON from Ollama
    used_files: Vec<String>,
    error: Option<String>,
}

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

// Helper to add CORS headers to any response
struct CorsResponder<R>(R);

impl<'r, 'o: 'r, R: Responder<'r, 'o>> Responder<'r, 'o> for CorsResponder<R> {
    fn respond_to(self, request: &'r Request<'_>) -> response::Result<'o> {
        let mut res = self.0.respond_to(request)?;

        // Add CORS headers
        res.set_header(Header::new("Access-Control-Allow-Origin", "*")); // or specific origin like "http://localhost:your-mvc-port"
        res.set_header(Header::new("Access-Control-Allow-Methods", "POST, OPTIONS"));
        res.set_header(Header::new("Access-Control-Allow-Headers", "Content-Type"));

        Ok(res)
    }
}

// New OPTIONS route to handle preflight
#[options("/query")]
fn options_handler() -> CorsResponder<Status> {
    CorsResponder(Status::Ok)
}

#[post("/query", format = "json", data = "<req>")]
async fn query(
    req: Json<QueryRequest>,
    _state: &State<Arc<()>>,  // placeholder; add data_dir later if needed
) -> Result<CorsResponder<Json<ApiResponse>>, status::Custom<String>> {
    let category = req.category.as_deref().unwrap_or("invoices").to_lowercase();

    let base_dir = match category.as_str() {
        "contracts" | "employment-contracts" => "data/employment-contracts",
        "support" | "customer-support"       => "data/customer-support",
        "knowledge" | "knowledge-base"       => "data/knowledge-base",
        _                                    => "data/invoices",  // default
    };

    let data_dir = std::path::Path::new(base_dir);

    if !data_dir.exists() {
        return Err(status::Custom(
            Status::BadRequest,
            format!("Category folder not found: {}", base_dir),
        ));
    }

    let relevant_files = find_relevant_files(data_dir, &req.query);
    if relevant_files.is_empty() {
        return Err(status::Custom(
            Status::BadRequest,
            "No relevant invoice files found".to_string(),
        ));
    }

    let mut contents = String::new();
    let mut file_names = Vec::new();

    for path in relevant_files {
        let text = match std::fs::read_to_string(&path) {
            Ok(t) => t,
            Err(e) => return Err(status::Custom(Status::InternalServerError, e.to_string())),
        };
        let fname = path.file_name().unwrap().to_string_lossy().to_string();
        contents.push_str(&format!("\n--- Invoice: {} ---\n{}\n", fname, text));
        file_names.push(fname);
    }

    match query_ollama("llama3.2", contents, &req.query, &category).await {
        Ok(raw_json) => {
            let parsed: serde_json::Value = match serde_json::from_str(&raw_json) {
                Ok(v) => v,
                Err(_) => serde_json::json!({"raw": raw_json}),
            };

            Ok(CorsResponder(Json(ApiResponse {
                answer: parsed,
                used_files: file_names,
                error: None,
            })))
        }
        Err(e) => Err(status::Custom(
            Status::InternalServerError,
            e.to_string(),
        )),
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(CORS)
        .configure(rocket::Config::figment().merge(("port", 8001)))
        .mount("/", routes![query, options_handler])
        .manage(Arc::new(()))  // can later hold config or shared state
}