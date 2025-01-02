use axum::Router;

use crate::bff::BffRouter;

pub fn get_admin_routers() -> BffRouter {
  let ret = Router::new();

  Router::new().nest("/admin", ret)
}
