mod account;
mod encrypt;
mod local;

use std::sync::Arc;

use anyhow::Result;
use axum::{routing, Router};
use tower::ServiceBuilder;
use tower_http::{
  compression,
  services::{ServeDir, ServeFile},
  trace,
};

use encrypt::Encrypt;

pub(super) struct BffParams {
  pub db: sea_orm::DatabaseConnection,

  /// Path to static resource like `frontend/dist/`
  pub static_path: String,

  /// Path prefix of static resource like `/__static`
  pub static_prefix: String,

  /// Optional encrypt secret key PEM file path
  pub pem_path: Option<String>,

  /// NAS IP in LAN
  pub lan_ip: String,
}

pub(super) struct BffGlobalData {
  encrypt: Encrypt,
  lan_ip: String,
  db: sea_orm::DatabaseConnection,
}

pub(super) type BffGlobalState = axum::extract::State<Arc<BffGlobalData>>;
pub(super) type BffRouter = Router<Arc<BffGlobalData>>;

/// 内网环境应答
fn post_ping() -> BffRouter {
  async fn handler() -> &'static str {
    "pong"
  }

  Router::new().route("/ping", routing::post(handler))
}

/// 创建bff - backend for frontend
pub fn get_bff(params: BffParams) -> Result<Router> {
  let BffParams {
    db,
    static_path,
    static_prefix,
    pem_path,
    lan_ip,
  } = params;

  let serve_index = ServeFile::new(format!("{}/index.html", &static_path));
  let serve_public = ServeDir::new(&static_path);

  let middleware = ServiceBuilder::new()
    .layer(trace::TraceLayer::new_for_http())
    .layer(compression::CompressionLayer::new());

  let encrypt = match pem_path {
    None => Encrypt::from_generate(),
    Some(path) => Encrypt::from_file(&path),
  }?;

  let bff_global_state = Arc::new(BffGlobalData {
    db,
    lan_ip,
    encrypt,
  });

  let ret = Router::new()
    .merge(post_ping())
    .merge(local::get_local_routers())
    .merge(account::get_account_routers())
    .with_state(bff_global_state);

  Ok(
    Router::new()
      .nest("/api", ret)
      .route_service("/", serve_index)
      .nest_service(&static_prefix, serve_public)
      .layer(middleware)
  )
}
