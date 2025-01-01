use axum::{extract::State, routing, Router};

use super::{BffGlobalState, BffRouter};

fn get_public_key() -> BffRouter {
  async fn handler(State(global): BffGlobalState) -> String {
    global.encrypt.get_public().to_string()
  }

  Router::new().route("/public_key", routing::get(handler))
}

fn get_lan_addr() -> BffRouter {
  async fn handler(State(global): BffGlobalState) -> String {
    global.lan_ip.clone()
  }

  Router::new().route("/addr", routing::get(handler))
}

pub fn get_local_routers() -> BffRouter {
  let routers = Router::new()
    .merge(get_lan_addr())
    .merge(get_public_key());

  Router::new().nest("/local", routers)
}
