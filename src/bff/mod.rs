mod encrypt;

mod account;

use std::sync::Arc;

use account::get_account_routers;
use anyhow::Result;
use axum::{routing, Router};
use tower_http::services::{ServeDir, ServeFile};

use encrypt::Encrypt;

pub struct BffParams<'a> {
  /// Path to static resource like `frontend/dist/`
  pub static_path: &'a str, 

  /// Path prefix of static resource like `/__static`
  pub static_prefix: &'a str,

  /// Optional encrypt secret key PEM file path
  pub pem_path: Option<&'a str>,
}

#[derive(Clone)]
struct BffGlobalState {
  encrypt: Arc<Encrypt>,
}

/// 内网环境应答
fn post_ping() -> Router<BffGlobalState> {
  async fn handler() -> &'static str {
    "pong"
  }

  Router::new().route("/ping", routing::post(handler))
}

/// 创建bff - backend for frontend
pub fn get_bff(params: BffParams) -> Result<Router> {
  let BffParams { static_path, static_prefix, pem_path } = params;

  let serve_index = ServeFile::new(format!("{}/index", &static_path));
  let serve_public = ServeDir::new(&static_path);

  let encrypt = match pem_path {
    None => Encrypt::from_generate(),
    Some(path) => Encrypt::from_file(&path),
  }?;

  let bff_global_state = BffGlobalState {
    encrypt: Arc::new(encrypt),
  };

  let ret = Router::new()
    .merge(post_ping())
    .merge(get_account_routers())
    .route_service("/", serve_index)
    .nest_service(&static_prefix, serve_public)
    .with_state(bff_global_state);

  Ok(Router::new().nest("/api", ret))
}
