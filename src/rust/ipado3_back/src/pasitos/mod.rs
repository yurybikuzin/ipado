use super::*;

use futures::StreamExt;
use server::{
    common::{
        request_message, request_message_sync, send_response_message, start_receive,
        RequestMessageResult, RxPasitos, TxHandle,
    },
    ws::common::{ping_ws_connections, start_ping_ws_connections},
    ResponseMessage,
};

// ==================================================
// ==================================================
// You have to customize:
// - here
// pub mod db;
pub mod spreadsheet;
// ==================================================
// ==================================================

pasitos!(fut_queue, run_for;
    init {
        let start = std::time::Instant::now();
        let opt = (*OPT.read().unwrap()).clone().unwrap();
        match opt.cmd.as_ref().unwrap() {
            Command::Server {..} => {
                start_receive();
                start_ping_ws_connections();
            },
        }
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

    // >>> server::ws
    demoras {
        demora WsPing({
            duration: tokio::time::Duration,
        }) {
            pasitos!(ws push_back Ping { duration });
        }
    }

    pasos ws {
        max_at_once: 1;
        paso Ping({
            duration: tokio::time::Duration,
        }) -> ({
        }) {
            ping_ws_connections(duration).await;
        } => sync {
            let duration = tokio::time::Duration::from_secs(settings!(ping_interval_secs));
            pasitos!(delay WsPing {duration} for duration);
        }

        // paso SendMessage({
        //     message: ServerMessage,
        //     send_message_mode: SendMessageMode,
        // }) -> ({
        // }) {
        //     todo!();
        //     // crate::server::ws::common::ping_ws_connections(duration).await;
        // } => sync {
        //     todo!();
        //     // let duration = tokio::time::Duration::from_secs(settings!(ping_interval_secs));
        //     // pasitos!(delay WsPing {duration} for duration);
        // }
    }
    // <<< server::ws

    // >>> server
    pasos receive {
        max_at_once: 1;
        paso RequestMessage({
            rx: RxPasitos,
        }) -> ({
            res: RequestMessageResult,
        }) {
            let res = request_message(rx).await;
        } => sync {
            request_message_sync(res)?;
        }
    }
    // <<< server

    // ==================================================
    // ==================================================
    // You have to customize:
    // - here
    // pasos db {
    //     max_at_once: settings!(db.connection_max_count) as usize;
    //
    //     paso Import({
    //         data: ImportRet,
    //         op_mode: Option<OpMode>,
    //     }) -> ({
    //         res: pasitos::db::ImportResult,
    //     }) {
    //         let res = pasitos::db::import(data, op_mode).await;
    //     } => sync {
    //         pasitos::db::import_sync(res)?;
    //     }
    //
    //     paso GetInitData({
    //         tx: TxHandle,
    //         key: InitDataKey,
    //     }) -> ( {
    //         res: pasitos::db::GetInitDataResult,
    //         tx: TxHandle,
    //     }) {
    //         let res = pasitos::db::get_init_data(key).await;
    //     } => sync {
    //         pasitos::db::get_init_data_sync(res, tx)?;
    //     }
    //     //
    //     // paso ImportInitData  ({
    //     //     title: String,
    //     //     // rows: Vec<ShortRow>,
    //     //     rows: Vec<Row>,
    //     //     tx: TxHandle,
    //     // }) -> ( {
    //     //     res: pasitos::db::GetInitDataResult,
    //     //     tx: TxHandle,
    //     // }) {
    //     //     let res = pasitos::db::import_init_data(title, rows).await;
    //     // } => sync {
    //     //     pasitos::db::import_init_data_sync(res, tx)?;
    //     // }
    //
    //     paso SetCursor  ({
    //         cursor: Option<i16>,
    //         user: String,
    //         event_hall: i32,
    //     }) -> ( {
    //         res: pasitos::db::SetCursorResult,
    //     }) {
    //         let res = pasitos::db::set_cursor(cursor, user, event_hall).await;
    //     } => sync {
    //         pasitos::db::set_cursor_sync(res)?;
    //     }
    // }

    pasos spreadsheet {
        max_at_once: 1;

        paso Import({
            // name: String,
            // op_mode: Option<OpMode>,
            // halls: bool,
        }) -> ({
            // name: String,
            // op_mode: Option<OpMode>,
            res: pasitos::spreadsheet::ImportResult,
        }) {
            // let res = pasitos::spreadsheet::import(halls).await;
            let res = pasitos::spreadsheet::import().await;
        } => sync {
            // pasitos::spreadsheet::import_sync(res, op_mode)?;
            pasitos::spreadsheet::import_sync(res)?;
        }

        paso GetInitData({
            tx: TxHandle,
            key: InitDataKey,
        }) -> ({
            tx: TxHandle,
            res: pasitos::spreadsheet::GetInitDataResult,
        }) {
            let res = pasitos::spreadsheet::get_init_data(key).await;
        } => sync {
            pasitos::spreadsheet::get_init_data_sync(res, tx)?;
        }
    }

    // ==================================================
    //
    // ==================================================
);
