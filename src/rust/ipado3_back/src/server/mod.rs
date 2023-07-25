use super::*;

use common_macros2::pasitos;
use warp::{Filter, Rejection, Reply};

#[macro_use]
pub mod common;
pub mod ws;

pub fn api() -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    ws::common::api()
        .or(common::health::api())
        .with(warp::trace::request())
}

#[derive(Debug, strum::EnumDiscriminants)]
#[strum_discriminants(derive(strum::Display))]
#[strum_discriminants(name(RequestMessageKind))]
pub enum RequestMessage {
    InitData { key: InitDataKey, refresh: bool },
}

#[derive(Debug, strum::EnumDiscriminants)]
#[strum_discriminants(derive(strum::Display))]
#[strum_discriminants(name(ResponseMessageKind))]
pub enum ResponseMessage {
    InitData(Result<Vec<Heat>>),
}

pub fn process_request_message(request_message: RequestMessage, tx: common::TxHandle) {
    match request_message {
        RequestMessage::InitData { refresh: _, key } => {
            pasitos!(spreadsheet push_back GetInitData { tx, key });
        }
    }
}
