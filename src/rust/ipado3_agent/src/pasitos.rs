use super::*;

use futures::StreamExt;

pub mod watcher;
pub mod ws;

const HOST: &str = "ipado.ru";
const BACK: &str = "ipado3_back";
const WATCHER_CHANNEL_SIZE: usize = 100;
pasitos!(fut_queue, run_for;
    init {
        let start = std::time::Instant::now();
        let opt = (*OPT.read().unwrap()).clone().unwrap();
        let url = OP_MODE.read().unwrap().ws_url(HOST, BACK);
        println!("will connect to {url:?}");
        let connect_retry_millis = opt.connect_retry_millis;
        pasitos!(ws push_back Connect { url, connect_retry_millis });

        let rx = pasitos::watcher::run(opt).await?;
        pasitos!(watcher push_back ReceiveEvent { rx });
    }
    on_complete {
        info!(
            "{}, complete",
            arrange_millis::get(std::time::Instant::now().duration_since(start).as_millis()),
        );
        return Ok(());
    }
    on_next_end {
    }
    demoras {
        demora WsConnect({
            url: String,
            connect_retry_millis: u64,
        }) {
            pasitos!(ws push_back Connect {
                url,
                connect_retry_millis,
            });
        }
    }

    pasos ws {
        max_at_once: 1;

        paso Connect({
            url: String,
            connect_retry_millis: u64,
        }) -> ({
            res: pasitos::ws::ConnectResult,
            url: String,
            connect_retry_millis: u64,
        }) {
            let res = pasitos::ws::connect(&url).await;
        } => sync {
            pasitos::ws::connect_sync(res, url, connect_retry_millis)?;
        }
    }

    pasos ws_read {
        max_at_once: 1;

        paso Read({
            ws: pasitos::ws::WsRead,
            url: String,
            connect_retry_millis: u64,
        }) -> ({
            res: pasitos::ws::ReadResult,
            url: String,
            connect_retry_millis: u64,
        }) {
            let res = pasitos::ws::read(ws).await;
        } => sync {
            pasitos::ws::read_sync(res, url, connect_retry_millis)?;
        }
    }

    pasos ws_write {
        max_at_once: 1;

        paso Perform({
            action: pasitos::ws::PerformAction,
            and_then: pasitos::ws::PerformAndThen,
        }) -> ({
            res: pasitos::ws::PerformResult,
            and_then: pasitos::ws::PerformAndThen,
        }) {
            let res = pasitos::ws::perform(action, &and_then).await;
        } => sync {
            pasitos::ws::perform_sync(res, and_then)?;
        }

        paso Send({
            frame: websockets::Frame,
        }) -> ({
            res: pasitos::ws::SendResult,
        }) {
            let res = pasitos::ws::send(frame).await;
        } => sync {
            pasitos::ws::send_sync(res)?;
        }
    }

    pasos watcher {
        max_at_once: 1;
        paso ReceiveEvent({
            rx: pasitos::watcher::Rx,
        }) -> ({
            res: pasitos::watcher::ReceiveEventResult,
        }) {
            let res = pasitos::watcher::receive_event(rx).await;
        } => sync {
            pasitos::watcher::receive_event_sync(res)?;
        }
    }
);
