use structopt::StructOpt;
use std::time::Duration;
use humantime::parse_duration;

#[derive(StructOpt, Debug)]
#[structopt(author = "t1t4m1un", about = "A better panel app for nas.")]
pub struct Config {
  #[structopt(
    long,
    help = "Help client to determine network environment",
  )]
  pub lan_ip: String,

  #[structopt(
    long,
    default_value = "./config",
    help = "Specify the path to the config directory",
  )]
  pub config_dir: String,

  #[structopt(
    long,
    default_value = "1y",
    help = "Specify the duration of login session",
    parse(try_from_str = parse_duration),
  )]
  pub login_duration: Duration,

  #[structopt(
    long,
    help = "Specify RSA secret PEM file",
  )]
  pub secret_file: Option<String>,
}
