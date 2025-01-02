use axum::Router;

use crate::bff::BffRouter;

pub fn get_user_routers() -> BffRouter {
  let ret = Router::new();

  Router::new().nest("/user", ret)
}
