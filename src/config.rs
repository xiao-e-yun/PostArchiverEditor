use std::path::PathBuf;

use clap::Parser;
use clap_verbosity_flag::InfoLevel;

#[derive(Debug, Clone, Parser)]
pub struct Config {
    #[clap(env = "ARCHIVER_PATH", default_value = "archive")]
    pub path: PathBuf,
    #[clap(long, default_value = "3000")]
    pub port: u16,
    #[command(flatten)]
    pub verbosity: clap_verbosity_flag::Verbosity<InfoLevel>,
}
