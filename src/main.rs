use config::Config;

use std::time;
use structopt::StructOpt;
use tokio::task::JoinHandle;

mod bff;
mod config;
mod entity;

async fn db_init(db_path: &str, config: &Config) -> sea_orm::DatabaseConnection {
  let db_url = format!("sqlite://{}?mode=rwc", db_path);
  let mut option = sea_orm::ConnectOptions::new(db_url);
  option
    .max_connections(64)
    .min_connections(4)
    .min_connections(5)
    .connect_timeout(time::Duration::from_secs(8))
    .acquire_timeout(time::Duration::from_secs(8))
    .idle_timeout(time::Duration::from_secs(8))
    .max_lifetime(time::Duration::from_secs(8))
    .sqlx_logging(true)
    .sqlx_logging_level(log::LevelFilter::Info);

  let db = sea_orm::Database::connect(option)
    .await
    .expect("Failed to connect to database");

  entity::init_table(&db, &config).await;

  db
}

fn bff_init(
  config: &Config,
  db: sea_orm::DatabaseConnection,
) -> JoinHandle<Result<(), std::io::Error>> {
  let Config {
    lan_ip,
    static_path,
    secret_file,
    port,
    ..
  } = config;

  let bff_service = bff::get_bff(bff::BffParams {
    db,
    lan_ip: lan_ip.clone(),
    static_path: static_path.clone(),
    pem_path: secret_file.clone(),
    static_prefix: "/__static".to_string(), // 该路径在vite.config.ts中已写死, 遂在rs中也写死, 自行重配置不难
  })
  .expect("Failed to create BFF service");

  let tcp_listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port));

  tokio::spawn(async { axum::serve(tcp_listener.await.unwrap(), bff_service).await })
}

#[tokio::main]
async fn main() {
  let config = Config::from_args_safe().unwrap_or_else(|e| {
    println!("{}", e.message);
    panic!("Please provide correct config");
  });

  let db = db_init(&config.db_path, &config).await;
  let bff_task = bff_init(&config, db.clone());

  let _ = tokio::join!(bff_task);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[tokio::test]
  async fn dev_server() -> Result<(), anyhow::Error> {
    tracing_subscriber::fmt::init();

    let config = Config {
      lan_ip: "127.0.0.1:3000".to_string(),
      default_admin: "admin".to_string(),
      default_password: "114514".to_string(),
      db_path: "dev.sqlite".to_string(),
      secret_file: None,
      static_path: "frontend/dist/".to_string(),
      port: 3000,
    };

    let dev_db = db_init("dev.sqlite", &config).await;
    let bff_task = bff_init(&config, dev_db);

    let _ = tokio::join!(bff_task);

    Ok(())
  }
}
