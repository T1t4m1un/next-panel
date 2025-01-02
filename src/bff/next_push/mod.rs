use axum::Router;

use super::BffRouter;

mod admin;
mod app;
mod user;

pub fn get_next_push_routers() -> BffRouter {
  let ret = Router::new()
    .merge(app::get_app_routers())
    .merge(admin::get_admin_routers())
    .merge(user::get_user_routers());

  Router::new().nest("/next_push", ret)
}
