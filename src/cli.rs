use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(long, env = "FRONIUS_HOST")]
    pub fronius_host: String,
    #[arg(long, default_value = "120", env = "FRONIUS_TIMEOUT_SEC")]
    pub fronius_timeout_sec: u32,
    #[arg(long, default_value = "10", env = "FRONIUS_UPDATE_SEC")]
    pub fronius_update_sec: u32,
    #[arg(long, default_value = "0.0.0.0:9123", env = "METRICS_BIND")]
    pub metric_bind: String,
}
