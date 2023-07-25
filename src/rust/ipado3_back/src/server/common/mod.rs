use super::*;

#[macro_use]
pub mod macros;
pub use macros::*;

pub mod error;

pub mod health;
// pub mod login;

use std::sync::RwLock;
use tokio::sync::mpsc::{channel, Receiver, Sender};

pub type TxPasitos = Sender<(Sender<ResponseMessage>, super::RequestMessage)>;

pub type RxPasitos = Receiver<(Sender<ResponseMessage>, RequestMessage)>;
pub type TxHandle = Sender<ResponseMessage>;
pub const CHANNEL_LEN: usize = 10000;

lazy_static::lazy_static! {
    pub static ref TX: RwLock<Option<TxPasitos>> = RwLock::new(None);
}

use serde::Serialize;
use std::convert::Infallible;
use warp::http::StatusCode;
use warp::{Rejection, Reply};

pub fn start_receive() {
    let (tx, rx) = channel::<(Sender<ResponseMessage>, RequestMessage)>(CHANNEL_LEN);
    *TX.write().unwrap() = Some(tx);
    pasitos!(receive push_back RequestMessage { rx });
}

pub fn send_response_message(response_message: ResponseMessage, tx: TxHandle) {
    let message_kind = ResponseMessageKind::from(&response_message);
    if let Err(err) = tx.try_send(response_message) {
        error!("failed to send response {:?}: {}", message_kind, err);
    }
}

pub struct RequestMessageResult(pub (TxHandle, RequestMessage), pub RxPasitos);
pub async fn request_message(mut rx: RxPasitos) -> RequestMessageResult {
    RequestMessageResult(rx.recv().await.unwrap(), rx)
}
pub fn request_message_sync(
    RequestMessageResult((tx, request_message), rx): RequestMessageResult,
) -> Result<()> {
    super::process_request_message(request_message, tx);
    pasitos!(receive push_back RequestMessage { rx });
    Ok(())
}

// This function receives a `Rejection` and tries to return a custom
// value, otherwise simply passes the rejection along.
pub async fn rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    let code;
    let message: String;
    if err.is_not_found() {
        code = StatusCode::NOT_FOUND;
        message = "NOT_FOUND".to_owned();
    } else if let Some(err) = err.find::<error::Error>() {
        code = err.into();
        message = err.to_string();
    } else if let Some(e) = err.find::<warp::filters::body::BodyDeserializeError>() {
        code = StatusCode::BAD_REQUEST;
        message = format!("{}", e);
    } else if let Some(_err) = err.find::<warp::reject::UnsupportedMediaType>() {
        code = StatusCode::BAD_REQUEST;
        message = "expected Content-type: application/json".to_owned();
    } else if let Some(_err) = err.find::<warp::reject::PayloadTooLarge>() {
        code = StatusCode::BAD_REQUEST;
        message = "request body is TOO LARGE".to_owned();
    } else if let Some(err) = err.find::<warp::reject::MethodNotAllowed>() {
        code = StatusCode::METHOD_NOT_ALLOWED;
        message = format!("{:?}", err);
    } else {
        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = format!("{:?}", err);
    }

    let json = {
        let reason = code.canonical_reason();
        let code = code.as_u16();
        match &reason {
            None => error!("{}: {}", code, message),
            Some(reason) => error!("{}: {}: {}", code, reason, message),
        }
        /// An API error serializable to JSON.
        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        pub enum ErrReply {
            Err {
                code: u16,
                reason: Option<&'static str>,
                message: String,
            },
        }
        warp::reply::json(&ErrReply::Err {
            code,
            reason,
            message,
        })
    };

    Ok(warp::reply::with_status(json, code))
}
