use clap::{Args, Parser};

#[derive(Args, Debug, Clone)]
#[group(multiple = false, required = true)]
pub struct HostDetect {
    #[arg(long, env = "FRONIUS_HOST")]
    pub fronius_host: Option<String>,
    #[arg(long, env = "FRONIUS_ZEROCONF")]
    pub fronius_zeroconf: bool,
}

#[derive(Parser, Debug)]
pub struct CliArgs {
    #[command(flatten)]
    pub fronius_detect: HostDetect,
    #[arg(long, default_value = "120", env = "FRONIUS_TIMEOUT_SEC")]
    pub fronius_timeout_sec: u32,
    #[arg(long, default_value = "10", env = "FRONIUS_UPDATE_SEC")]
    pub fronius_update_sec: u32,
    #[arg(long, default_value = "0.0.0.0:9123", env = "METRICS_BIND")]
    pub metric_bind: String,
}
