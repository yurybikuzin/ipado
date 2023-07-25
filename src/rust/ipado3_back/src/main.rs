#[allow(unused_imports)]
use anyhow::{anyhow, bail, Context, Error, Result};
#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

// https://docs.rs/built/0.5.1/built/index.html
pub mod built_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}
use built_info::*;
use common_macros2::*;
use serde::{Deserialize, Serialize};
use structopt::StructOpt;

mod common;
mod pasitos;
mod server;
use common::*;

#[tokio::main]
async fn main() -> Result<()> {
    main_helper().await
}

// ==================================================
// ==================================================

use ipado3_common::*;
use std::path::PathBuf;

declare_env_settings_for_server! {
    settings_toml_path: std::path::PathBuf,
}

declare_settings! {
    ping_interval_secs: u64,
    cache_for_local: Option<PathBuf>,
    login: SettingsLogin,
    spreadsheet: SettingsContentSpreadsheet,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SettingsContentSpreadsheet {
    pub service_account_secret_file: String,
    pub id: String,
    pub name_range: String,
    pub schedule_range: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SettingsLogin {
    pub token_url: String,
    pub secret_key: String,
}

#[derive(Debug, Clone, StructOpt)]
pub enum Command {
    Server {
        #[structopt(short, long)]
        port: Option<u16>,
        #[structopt(long)]
        op_mode: Option<op_mode::OpMode>,
    },
}

// ==================================================
// ==================================================
