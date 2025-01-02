use axum::Router;

use super::BffRouter;

mod login;

pub fn get_account_routers() -> BffRouter {
  let ret = Router::new()
    .merge(login::post_login());

  Router::new().nest("/account", ret)
}
