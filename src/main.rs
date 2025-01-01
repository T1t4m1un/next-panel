use config::Config;
use structopt::StructOpt;

mod bff;
mod config;

#[tokio::main]
async fn main() {
  let config = Config::from_args_safe()
    .unwrap_or_else(|e| {
      println!("{}", e.message);
      panic!("Please provide correct config");
    });
}

#[cfg(test)]
mod tests {

  use super::*;
  use bff::BffParams;
  use tokio::net::TcpListener;

  #[tokio::test]
  async fn dev_server() -> Result<()> {
    let bff_params = BffParams {
      pem_path: None,
      static_path: "frontend/dist",
      static_prefix: "/__static",
    };

    let bff = bff::get_bff(bff_params)?;

    let dev_server = axum::serve(TcpListener::bind("0.0.0.0:3333").await?, bff);
    let dev_task = tokio::spawn(async { dev_server.await });

    let _ = tokio::join!(dev_task);

    Ok(())
  }
}
