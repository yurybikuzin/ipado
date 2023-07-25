use super::*;

use futures::SinkExt;
use futures::StreamExt;
use futures::TryFutureExt;
use std::collections::HashMap;
use std::collections::HashSet;
use std::sync::RwLock;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;

#[macro_use]
pub mod macros;
pub use macros::*;

// pub mod login;

pub fn api() -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Copy {
    warp::get()
        .and(warp::path("ws"))
        .and(warp::path::end())
        .and(warp::ws())
        .map(|ws: warp::ws::Ws| ws.on_upgrade(super::common::ws_connection_establised))
}

pub type WsConnections = Arc<std::sync::RwLock<HashMap<WsConnectionId, WsConnection>>>;

pub fn start_ping_ws_connections() {
    if !matches!(*OP_MODE.read().unwrap(), op_mode::OpMode::Local) {
        let duration = tokio::time::Duration::from_secs(settings!(ping_interval_secs));
        pasitos!(delay WsPing {duration} for duration);
    }
}

use std::time::Instant;
pub struct WsConnection {
    tx: mpsc::UnboundedSender<Message>,
    last_pong: Instant,
    pub kind: Option<Kind>,
    #[allow(dead_code)]
    key: Option<String>,
}

#[allow(dead_code)]
pub enum Send {
    ToSelf,
    All,
    AllButSelf,
    To(HashSet<WsConnectionId>),
}

static NEXT_WS_CONNECTION_ID: AtomicUsize = AtomicUsize::new(1);

lazy_static::lazy_static! {
    pub static ref WS_CONNECTIONS: WsConnections = WsConnections::default();
    // pub static ref INIT_DATA_STATUS: RwLock<HashMap<InitDataKey, InitDataStatus>> = RwLock::new(HashMap::new());
    pub static ref INIT_DATA_STATUS: RwLock<HashMap<InitDataKey, InitDataStatus>> = RwLock::new(HashMap::new());
}

#[derive(Clone)]
pub enum InitDataStatus {
    Ok(Arc<RwLock<Vec<Heat>>>),
    Err { at: std::time::Instant },
    Wait(HashSet<WsConnectionId>),
}

use futures::stream::{SplitSink, SplitStream};
use warp::ws::{Message, WebSocket};

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct WsConnectionId(usize);
impl WsConnectionId {
    fn new() -> Self {
        Self(NEXT_WS_CONNECTION_ID.fetch_add(1, Ordering::Relaxed))
    }
}

pub async fn ws_connection_establised(ws: WebSocket) {
    let ws_connection_id = WsConnectionId::new();
    debug!("new ws_connection {ws_connection_id:?}");
    let (ws_tx, ws_rx) = ws.split();
    let (tx, rx) = mpsc::unbounded_channel();
    spawn_ws_sender(ws_tx, rx);
    WS_CONNECTIONS.write().unwrap().insert(
        ws_connection_id,
        WsConnection {
            tx,
            last_pong: Instant::now(),
            kind: None,
            key: None,
        },
    );
    process_ws_connection_messages(ws_connection_id, ws_rx).await;
    WS_CONNECTIONS.write().unwrap().remove(&ws_connection_id);
    debug!("lost ws_connection {ws_connection_id:?}");
}

pub async fn ping_ws_connections(duration: tokio::time::Duration) {
    for (ws_connection_id, tx) in WS_CONNECTIONS.read().unwrap().iter().filter_map(
        |(ws_connection_id, WsConnection { tx, last_pong, .. })| {
            (Instant::now().duration_since(*last_pong) >= duration)
                .then_some((ws_connection_id, tx))
        },
    ) {
        if let Err(err) = tx.send(Message::ping(vec![])) {
            warn!("{}:{}: ping {ws_connection_id:?}: {err}", file!(), line!());
        }
    }
}

fn spawn_ws_sender(
    mut ws_tx: SplitSink<warp::ws::WebSocket, warp::ws::Message>,
    rx: tokio::sync::mpsc::UnboundedReceiver<Message>,
) {
    let mut rx = UnboundedReceiverStream::new(rx);
    tokio::task::spawn(async move {
        while let Some(message) = rx.next().await {
            ws_tx
                .send(message)
                .unwrap_or_else(|e| {
                    error!("websocket send error: {}", e);
                })
                .await;
        }
    });
}

async fn process_ws_connection_messages(
    ws_connection_id: WsConnectionId,
    mut ws_rx: SplitStream<warp::ws::WebSocket>,
) {
    while let Some(result) = ws_rx.next().await {
        let msg = match result {
            Ok(msg) => msg,
            Err(err) => {
                error!("websocket error(uid={ws_connection_id:?}): {err}");
                break;
            }
        };
        if msg.is_pong() {
            if let Some(WsConnection { last_pong, .. }) =
                WS_CONNECTIONS.write().unwrap().get_mut(&ws_connection_id)
            {
                *last_pong = Instant::now();
            } else {
                warn!(
                    "{}:{}: pong for absent connection: {ws_connection_id:?}",
                    file!(),
                    line!()
                );
            }
        } else if msg.is_ping() {
            if let Some(WsConnection { last_pong, tx, .. }) =
                WS_CONNECTIONS.write().unwrap().get_mut(&ws_connection_id)
            {
                *last_pong = Instant::now();
                trace!(
                    "{}:{}: ping: {ws_connection_id:?}: last_pong: {last_pong:?}",
                    file!(),
                    line!(),
                );
                let _ = tx.send(Message::pong(vec![])); // https://docs.rs/warp/latest/warp/filters/ws/struct.Message.html#method.pong
            } else {
                warn!("{}:{}: ping: {ws_connection_id:?}", file!(), line!());
            }
        } else {
            process_ws_connection_message(ws_connection_id, msg).await;
        }
    }
}

pub fn respond_to_ping() -> Option<(ServerMessage, Send)> {
    Some((ServerMessage::Pong, Send::ToSelf))
}

pub fn respond_to_version(client_message: ClientMessage) -> Option<(ServerMessage, Send)> {
    if let ClientMessage::Version(version_client) = client_message {
        let version_server = semver::Version::parse(PKG_VERSION).unwrap();
        let res = if version_client.major == version_server.major
            && version_client.minor == version_server.minor
        {
            let need_refresh = false;
            Ok(need_refresh)
        } else if version_client.major >= version_server.major
            || version_client.minor > version_server.minor
        {
            Err(format!(
                "version_client: {version_client:?}, version_server: {version_server:?}"
            ))
        } else {
            let need_refresh = true;
            Ok(need_refresh)
        };
        Some((ServerMessage::Version(res), Send::ToSelf))
    } else {
        unreachable!();
    }
}

// pub const RE_HREF_STR: &str = r#"(?x)
//     ^
//         https://
//             (?P<host> [^/?\#]+ )
//         (?:
//             (?P<pathname> / [^?\#]* )
//         )?
//         (?: [?]
//             (?P<query> [^\#]* )
//         )?
//         (?: \#
//             (?P<hash> .* )
//         )?
//     $
//     "#;

// const RE_HREF_STR: &str = r#"(?x) ^https://(?P<host>[^/?#]+)(?:/(?P<pathname>[^?#]*))?(?:[?](?P<query>[^#]*))?(?:#(?P<hash>.*))?$"#;

// lazy_static::lazy_static! {
//     pub static ref RE_HREF: regex::Regex = {
//         match regex::Regex::new(RE_HREF_STR) {
//             Ok(ret) => ret,
//             Err(err) => panic!("{}:{}: {err}", file!(), line!()),
//         }
//     };
// }

pub fn respond_to_init(
    client_message: ClientMessage,
    _ws_connection_id: WsConnectionId,
    need_send_init_data_after_all: &mut Option<InitDataKey>,
) -> Option<(ServerMessage, Send)> {
    let ClientMessageInit {
        auth,
        key,
        location,
    } = client_message.init_get().unwrap();
    if NO_GUEST || location.pathname.ends_with("/admin/") {
        Some((
            if let Some(auth) = {
                if location.host.starts_with("local.") {
                    (*LOCAL_AUTH).clone()
                } else {
                    auth
                }
            } {
                *need_send_init_data_after_all = Some(key);
                ServerMessage::login_set(Some(Ok(auth)))
            } else {
                // ServerMessage::NeedLogin
                *need_send_init_data_after_all = Some(key);
                ServerMessage::login_set(Some(Ok((*LOCAL_AUTH).clone().unwrap())))
            },
            Send::ToSelf,
        ))
    } else {
        *need_send_init_data_after_all = Some(key);
        None
    }
}

// pub async fn respond_to_login(
//     client_message: ClientMessage,
//     ws_connection_id: WsConnectionId,
// ) -> Option<(ServerMessage, Send)> {
//     if let ClientMessage::Login(message) = client_message {
//         Some((login::login(ws_connection_id, message).await, Send::ToSelf))
//     } else {
//         unreachable!();
//     }
// }

pub async fn respond_to_need_init_data(
    client_message: ClientMessage,
    ws_connection_id: WsConnectionId,
) -> Option<(ServerMessage, Send)> {
    if let ClientMessage::NeedInitData { key, refresh } = client_message {
        enum Need {
            Fetch,
            // ResponseOk(Arc<RwLock<InitData>>),
            ResponseOk(InitData),
            Wait,
        }
        let need_opt: Option<Need> = match INIT_DATA_STATUS.read().unwrap().get(&key) {
            None => Some(Need::Fetch),
            Some(InitDataStatus::Ok(heatlar)) => Some(if refresh {
                Need::Fetch
            } else {
                // debug!(
                //     "{}:{}: init_data.{key:?}.cursor: {:?}",
                //     file!(),
                //     line!(),
                //     init_data.read().unwrap().cursor
                // );
                Need::ResponseOk(InitData {
                    heatlar: (*heatlar.read().unwrap()).clone(),
                    heat_details: (*HEAT_DETAILS.read().unwrap()).clone(),
                    cursor: *CURSOR.read().unwrap(),
                    dance: (*DANCE.read().unwrap()).clone(),
                })
            }),
            Some(InitDataStatus::Err { at }) => {
                if Instant::now().duration_since(*at) > std::time::Duration::from_secs(60) {
                    Some(Need::Fetch)
                } else {
                    None
                }
            }
            Some(InitDataStatus::Wait(_)) => Some(Need::Wait),
        };
        let ret: Option<(InitData, Send)> = match need_opt {
            None => None,
            Some(Need::Fetch) => {
                debug!("will fetch");
                INIT_DATA_STATUS.write().unwrap().insert(
                    key.clone(),
                    InitDataStatus::Wait(vec![ws_connection_id].into_iter().collect()),
                );
                let init_data_res_opt = match send_receive!(RequestMessage::InitData{refresh, key: key.clone()} => ResponseMessage::InitData(res) => res)
                {
                    Err(err) => {
                        error!("{}:{}: {err:?}", file!(), line!());
                        None
                    }
                    Ok(Err(err)) => {
                        error!("{}:{}: {err:?}", file!(), line!());
                        Some(Err(()))
                    }
                    Ok(Ok(init_data)) => Some(Ok(Arc::new(RwLock::new(init_data)))),
                };
                let ret_opt: Option<(InitData, HashSet<WsConnectionId>)> =
                    if let Some(init_data_res) = init_data_res_opt {
                        let (init_data_opt, init_data_status) = match init_data_res {
                            Err(()) => (None, InitDataStatus::Err { at: Instant::now() }),
                            Ok(init_data) => {
                                (Some(init_data.clone()), InitDataStatus::Ok(init_data))
                            }
                        };
                        let init_data_status_opt = INIT_DATA_STATUS.write().unwrap().remove(&key);
                        if NEED_CACHE_INIT_DATA {
                            INIT_DATA_STATUS
                                .write()
                                .unwrap()
                                .insert(key, init_data_status);
                        }
                        init_data_opt.map(|heatlar| {
                            (
                                InitData {
                                    heatlar: (*heatlar.read().unwrap()).clone(),
                                    heat_details: (*HEAT_DETAILS.read().unwrap()).clone(),
                                    cursor: *CURSOR.read().unwrap(),
                                    dance: (*DANCE.read().unwrap()).clone(),
                                },
                                match init_data_status_opt {
                                    Some(InitDataStatus::Wait(ids)) => ids,
                                    _ => unreachable!(),
                                },
                            )
                        })
                    } else {
                        None
                    };
                ret_opt.map(|(init_data, ids)| {
                    (init_data, if refresh { Send::All } else { Send::To(ids) })
                })
            }
            Some(Need::ResponseOk(init_data)) => Some((init_data, Send::ToSelf)),
            Some(Need::Wait) => {
                if let Some(InitDataStatus::Wait(ids)) =
                    &mut INIT_DATA_STATUS.write().unwrap().get_mut(&key)
                {
                    ids.insert(ws_connection_id);
                } else {
                    unreachable!();
                }
                None
            }
        };
        ret.map(|(init_data, send)| {
            // let init_data = InitData {
            //     heatlar,
            //     heat_details: (*HEAT_DETAILS.read().unwrap()).clone(),
            //     cursor: (*CURSOR.read().unwrap()).clone(),
            // };
            //     init_data.read().unwrap().clone();
            // init_data.heat_details = HEAT_DETAILS.read().unwrap().clone();
            // let ids = match &send {
            //     Send::ToSelf => {
            //         vec![ws_connection_id]
            //     }
            //     Send::To(ids) => ids.iter().cloned().collect(),
            //     _ => unreachable!(),
            // };
            // for ws_connection_id in ids.into_iter() {
            //     if let Some(WsConnection { kind, .. }) =
            //         WS_CONNECTIONS.write().unwrap().get_mut(&ws_connection_id)
            //     {
            //         if kind.is_none() {
            //             *kind = Some(Kind::Observer(init_data.турнир.clone()))
            //         }
            //     }
            // }
            (ServerMessage::init_data_set(&init_data), send)
        })
    } else {
        unreachable!();
    }
}

pub async fn post_process(
    need_send_init_data_after_all: Option<InitDataKey>,
    ws_connection_id: WsConnectionId,
) {
    if let Some(key) = need_send_init_data_after_all {
        debug!("{}:{}: need_send_init_data_after_all", file!(), line!());
        if let Some((message, send)) = common::respond_to_need_init_data(
            ClientMessage::NeedInitData {
                key,
                refresh: false,
            },
            ws_connection_id,
        )
        .await
        {
            debug!("{}:{}: will send_init_data_after_login", file!(), line!());
            send_message_to(message, send, ws_connection_id).await
        } else {
            debug!(
                "{}:{}: will wait before send_init_data_after_login",
                file!(),
                line!()
            );
        }
    }
}

pub async fn send_message_to(message: ServerMessage, send: Send, ws_connection_id: WsConnectionId) {
    let send_message_mode = match send {
        Send::ToSelf => SendMessageMode::To(vec![ws_connection_id].into_iter().collect()),
        Send::To(ids) => SendMessageMode::To(ids),
        Send::AllButSelf => SendMessageMode::AllBut {
            except: Some(vec![ws_connection_id].into_iter().collect()),
        },
        Send::All => SendMessageMode::AllBut { except: None },
    };
    send_message(message, send_message_mode).await;
}

pub enum SendMessageMode {
    To(HashSet<WsConnectionId>),
    // All,
    AllBut {
        except: Option<HashSet<WsConnectionId>>,
    },
}

pub async fn send_message(message: ServerMessage, send_message_mode: SendMessageMode) {
    let encoded = message.encoded();
    match send_message_mode {
        SendMessageMode::To(ids) => {
            for tx in WS_CONNECTIONS.read().unwrap().iter().filter_map(
                |(ws_connection_id, WsConnection { tx, .. })| {
                    if ids.contains(ws_connection_id) {
                        Some(tx)
                    } else {
                        None
                    }
                },
            ) {
                let _ = tx.send(Message::binary(encoded.clone()));
            }
        }
        SendMessageMode::AllBut { except } => {
            for tx in WS_CONNECTIONS.read().unwrap().iter().filter_map(
                |(ws_connection_id, WsConnection { tx, .. })| {
                    if let Some(except) = &except {
                        (!except.contains(ws_connection_id)).then_some(tx)
                    } else {
                        Some(tx)
                    }
                },
            ) {
                let _ = tx.send(Message::binary(encoded.clone()));
            }
        } // SendMessageMode::All => {
          //     for tx in WS_CONNECTIONS.read().unwrap().iter() {
          //         let _ = tx.send(Message::binary(encoded.clone()));
          //     }
          // }
    }
}
