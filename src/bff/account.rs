use axum::{extract::State, routing, Router};

use super::BffGlobalState;

fn get_public_key() -> Router<BffGlobalState> {
  async fn handler(State(BffGlobalState { encrypt, .. }): State<BffGlobalState>) -> String {
    encrypt.get_public().to_string()
  }

  Router::new().route("/public_key", routing::get(handler))
}

pub fn get_account_routers() -> Router<BffGlobalState> {
  let routers = Router::new()
    .merge(get_public_key());

  Router::new().nest("/account", routers)
}
