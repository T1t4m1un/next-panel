use axum::Router;

use super::BffRouter;

pub fn get_app_routers() -> BffRouter {
  let ret = Router::new();

  Router::new().nest("/app", ret)
}
