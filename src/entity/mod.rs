use openssl::{
  base64,
  hash::{Hasher, MessageDigest},
};
use sea_orm::{sqlx::types::chrono, ActiveModelTrait, ConnectionTrait, DatabaseConnection, Schema, Set};

use crate::config::Config;

pub mod user;

pub async fn init_table(db: &DatabaseConnection, config: &Config) {
  let builder = db.get_database_backend();
  let schema = Schema::new(builder);

  let stmt = builder.build(&schema.create_table_from_entity(user::Entity));
  let exec_res = db.execute(stmt).await;

  // Seed table
  if let Ok(_) = exec_res {
    let mut hasher = Hasher::new(MessageDigest::sha256()).unwrap();
    hasher.update(config.default_password.as_bytes()).unwrap();
    let digest: &[u8] = &hasher.finish().unwrap();
    let password = base64::encode_block(digest);

    let admin = user::ActiveModel {
      username: Set(config.default_admin.to_owned()),
      password: Set(password.to_owned()),
      role: Set("admin".to_owned()),
      create_at: Set(chrono::Utc::now()),
      ..Default::default()
    };

    let res = admin.insert(db).await;
    log::info!("{:?}", res);
  }
}
