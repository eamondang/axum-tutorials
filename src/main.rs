use std::net::SocketAddr;

use axum::{
  extract::{Path, Query},
  headers::UserAgent,
  http::{header, HeaderMap},
  response::IntoResponse,
  routing::{get, post},
  Form, Json, Router, Server, TypedHeader,
};
use serde::{Deserialize, Serialize};

#[tokio::main]
#[allow(unused)]
async fn main() {
  let app = Router::new()
    .route("/", get(say_hello))
    .route("/:msg", get(greeting))
    .route("/subject", get(query))
    .route("/head", get(get_headers))
    .route("/user", get(get_agent_user))
    .route("/user_req", get(get_user_req))
    .route("/user_req_json", get(json_req_json))
    .route("/user_res", post(respon_res))
    .route("/create", post(form_req))
    .route("/createjson", post(json_req));

  let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
  Server::bind(&addr).serve(app.into_make_service()).await;
}

async fn say_hello() -> String {
  "Hello World!!!".to_string()
}

async fn greeting(Path(msg): Path<String>) -> String {
  format!("User info for {}", msg)
}

#[derive(Deserialize)]
struct SubjectArg {
  pub page: i32,
  pub keyword: String,
}

async fn query(msg: Option<Query<SubjectArg>>) -> String {
  if let Some(msg) = msg {
    let msg = msg.0;
    return format!("Page {}, keyword: {}", msg.page, msg.keyword);
  }
  "Page 0, no Keyword".to_string()
}

#[derive(Deserialize, Serialize)]
pub struct CreateUser {
  pub id: u32,
  pub name: String,
}

async fn form_req(Form(form): Form<CreateUser>) -> String {
  format!("Created User: {}, name: {}", form.id, form.name)
}

async fn json_req(Json(msg): Json<CreateUser>) -> String {
  format!("Created User: {}, name: {}", msg.id, msg.name)
}

async fn get_headers(headers: HeaderMap) -> String {
  format!("headers: {:#?}", headers)
}

async fn get_agent_user(headers: HeaderMap) -> String {
  headers.get(header::USER_AGENT).and_then(|v| v.to_str().ok()).map(|v| v.to_string()).unwrap()
}

async fn get_user_req(TypedHeader(header): TypedHeader<UserAgent>) -> String {
  header.to_string()
}

async fn json_req_json(Json(msg): Json<CreateUser>) -> Json<CreateUser> {
  let user = CreateUser { id: msg.id, name: msg.name };
  Json(user)
}

async fn respon_res(Json(msg): Json<CreateUser>) -> impl IntoResponse {
  let user = CreateUser { id: msg.id, name: msg.name };
  Json(user).into_response()
}
