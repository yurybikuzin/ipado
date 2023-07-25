use super::*;
use structopt::StructOpt;

lazy_static::lazy_static! {
    pub static ref OPT: RwLock<Option<Opt>> = RwLock::new(None);
}

#[derive(Debug, StructOpt, Clone)]
#[structopt(name = PKG_NAME)]
pub struct Opt {
    /// Workdir where to read .env
    #[structopt(short, long, parse(from_os_str))]
    pub workdir: Option<PathBuf>,

    /// Test config
    #[structopt(short, long)]
    pub test_config: bool,

    /// No show opts
    #[structopt(short, long)]
    pub no_show_opts: bool,
    #[structopt(long)]
    pub op_mode: Option<OpMode>,

    /// workdir dir if not specified
    pub path_to_be_watched: Option<PathBuf>,
    #[structopt(short = "d", long, default_value = "100")]
    pub debounce_millis: u64,
    #[structopt(short = "e", long, default_value = "heats")]
    pub filter_by_ext: String,
    #[structopt(short = "u", long)]
    pub websocket_url: Option<String>,
    #[structopt(short = "r", long, default_value = "5000")]
    pub connect_retry_millis: u64,
}
