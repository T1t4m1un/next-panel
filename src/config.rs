use structopt::StructOpt;

#[derive(StructOpt, Debug, Clone)]
#[structopt(author = "t1t4m1un", about = "A better panel app for nas.")]
pub struct Config {
  #[structopt(
    long,
    help = "Help client to determine network environment",
  )]
  pub lan_ip: String,

  // #[structopt(
  //   long,
  //   default_value = "./config",
  //   help = "Specify the path to the config directory",
  // )]
  // pub config_dir: String,

  #[structopt(
    long,
    default_value = "admin",
  )]
  pub default_admin: String,

  #[structopt(
    long,
    default_value = "114514",
  )]
  pub default_password: String,

  #[structopt(
    long,
    default_value = "./db.sqlite",
    help = "Specify the path to db file",
  )]
  pub db_path: String,

  #[structopt(
    long,
    help = "Specify RSA secret PEM file",
  )]
  pub secret_file: Option<String>,

  #[structopt(
    long,
    default_value = "frontend/dist",
    help = "Specify frontend static resource path",
  )]
  pub static_path: String,

  #[structopt(
    long,
    default_value = "3000",
    help = "Specify the service port"
  )]
  pub port: i32,
}
