#[allow(unused_imports)]
use anyhow::{anyhow, bail, Context, Error, Result};

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

// https://docs.rs/built/0.5.1/built/index.html
pub mod built_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}
pub use built_info::*;

use chrono::{DateTime, SecondsFormat, Utc};
use chrono_tz::Europe::Moscow;
use common_macros2::*;
use ipado3_common::{AgentFileChanged, AgentInit, ClientMessage, ServerMessage};
use op_mode::OpMode;
use std::collections::HashMap;
use std::convert::From;
use std::path::PathBuf;
use std::sync::RwLock;
use structopt::StructOpt;
use websockets::{
    Frame, WebSocket,
    WebSocketError::{self, *},
    WebSocketReadHalf, WebSocketWriteHalf,
};

mod opt;
use opt::*;

mod pasitos;

lazy_static::lazy_static! {
    pub static ref OP_MODE: std::sync::RwLock<op_mode::OpMode> = std::sync::RwLock::new(op_mode::OpMode::default());
}

lazy_static::lazy_static! {
    pub static ref SESSION: RwLock<DateTime<Utc>> = RwLock::new(Utc::now());
    pub static ref AGENT_INIT_TO_SEND: tokio::sync::RwLock<Option<AgentInit>> = tokio::sync::RwLock::new(None);
    pub static ref WS: tokio::sync::RwLock<Option<WebSocketWriteHalf>> = tokio::sync::RwLock::new(None);
    pub static ref FILES: tokio::sync::RwLock<HashMap<String, u32>> = tokio::sync::RwLock::new(HashMap::new());
}

pub async fn send_agent_init() -> Result<()> {
    if let Some(ws) = (*WS.write().await).as_mut() {
        if let Some(data) = AGENT_INIT_TO_SEND.write().await.take() {
            use flate2::write::GzEncoder;
            use flate2::Compression;
            use std::io::prelude::*;
            let mut e = GzEncoder::new(Vec::new(), Compression::default());
            let encoded = bincode::serialize(&data)
                .map_err(|err| format!("{err}"))
                .and_then(|encoded| {
                    e.write_all(&encoded)
                        .map_err(|err| format!("{err}"))
                        .and_then(|_| e.finish().map_err(|err| format!("{err}")))
                });
            let message = ClientMessage::AgentInit(encoded);
            let frame = websockets::Frame::Binary {
                payload: message.encoded(),
                continuation: false,
                fin: true,
            };
            ws.send(frame).await?;
            info!("did send agent init data");
        } else {
            warn!("send_agent_init: no data");
        }
    } else {
        warn!("send_agent_init: no connection");
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("{PKG_NAME} {PKG_VERSION}");
    let opt = Opt::from_args();
    if let Some(workdir) = &opt.workdir {
        std::env::set_current_dir(workdir)
            .map_err(|err| anyhow!("failed to set {:?} for current dir: {}", opt.workdir, err))?;
    } else {
        let current_exe = std::env::current_exe()?;
        let workdir = current_exe.parent().unwrap();
        std::env::set_current_dir(workdir)
            .map_err(|err| anyhow!("failed to set {:?} for current dir: {}", opt.workdir, err))?;
    };
    if dotenv::dotenv().is_ok() {
        println!(
            "INFO: found {}/.env with RUST_LOG={:?}",
            std::env::current_dir().unwrap().display(),
            std::env::var("RUST_LOG")
        );
    }
    pretty_env_logger::init_timed();
    if !opt.no_show_opts {
        info!(
            "{} {}\ncurrent dir: {:?}",
            built_info::PKG_NAME,
            built_info::PKG_VERSION,
            std::env::current_dir().unwrap(),
        );
    }
    if opt.test_config {
        return Ok(());
    }

    *OP_MODE.write().unwrap() = OpMode::get_actual(&opt.op_mode);
    *(OPT.write().unwrap()) = Some(opt);

    pasitos::pasos::run().await?;
    Ok(())
}
