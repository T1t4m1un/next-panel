use axum::{
  extract::{self, State},
  routing, Json, Router,
};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

use super::{BffGlobalState, BffRouter};
use crate::entity::user::{self, Entity as User};

#[derive(serde::Deserialize)]
struct LoginRequestSchema {
  pub fp: String,
  pub username: String,
  pub password: String,
}

#[derive(serde::Serialize)]
struct JwtPayload {
  pub username: String,
  pub role: String,
  pub fp: String,
}

#[derive(serde::Serialize)]
struct LoginResponseSchema {
  pub status: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub info: Option<JwtPayload>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub sign: Option<String>,
}

impl LoginResponseSchema {
  fn from_fail(status: &str) -> Self {
    LoginResponseSchema {
      status: status.to_string(),
      info: None,
      sign: None,
    }
  }
}

fn post_login() -> BffRouter {
  async fn handler(
    State(data): BffGlobalState,
    extract::Json(request): extract::Json<LoginRequestSchema>,
  ) -> Json<LoginResponseSchema> {
    let query = User::find()
      .filter(user::Column::Username.eq(&request.username))
      .filter(user::Column::Password.eq(&request.password))
      .one(&data.db)
      .await;
    log::error!("{:?}", query);
    match query {
      Err(_) => Json(LoginResponseSchema::from_fail("error")),
      Ok(res) => match res {
        None => Json(LoginResponseSchema::from_fail("fail")),
        Some(user) => {
          let payload = JwtPayload {
            username: user.username,
            role: user.role,
            fp: request.fp,
          };
          let pl_json = serde_json::to_string(&payload).unwrap();
          let token = data.encrypt.sign_with_private_key(&pl_json);
          match token {
            Err(_) => Json(LoginResponseSchema::from_fail("error")),
            Ok(token) => Json(LoginResponseSchema {
              status: "success".to_string(),
              info: Some(payload),
              sign: Some(token),
            }),
          }
        }
      },
    }
  }

  Router::new().route("/login", routing::post(handler))
}

pub fn get_account_routers() -> BffRouter {
  let ret = Router::new()
    .merge(post_login());

  Router::new().nest("/account", ret)
}
