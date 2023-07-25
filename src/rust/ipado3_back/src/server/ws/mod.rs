use super::*;

pub mod common;
use common::*;

pub const NEED_CACHE_INIT_DATA: bool = false;
pub const NO_GUEST: bool = false;
lazy_static::lazy_static! {
    pub static ref LOCAL_AUTH: Option<login_export::AuthRet> = Some(login_export::AuthRet {
        contact: login_export::AuthContact::Email("yury.bikuzin@gmail.com".to_owned()),
        details: login_export::AuthRetDetails {
            nickname: None,
            name: None,
            given_name: None,
            middle_name: None,
            family_name: None,
            birthday: None,
            phone_number: None,
            email: None,
            emails: None,
            gender: None,
            picture: None,
        },
    });
}

pub enum Kind {
    // User(login_export::AuthRet),
}

async fn process_ws_connection_message(ws_connection_id: WsConnectionId, msg: warp::ws::Message) {
    if msg.as_bytes().is_empty() {
        warn!("{}:{}: msg.as_bytes().is_empty()", file!(), line!());
    } else {
        let mut need_send_init_data_after_all: Option<InitDataKey> = None;
        if let Some((message, send)) = match ClientMessage::from_encoded(msg.as_bytes()) {
            Err(err) => {
                error!("{}:{}: {err}", file!(), line!());
                None
            }
            Ok(client_message) => {
                let client_message_discriminants =
                    ClientMessageDiscriminants::from(&client_message);
                if !matches!(client_message, ClientMessage::Ping) {
                    debug!("got {client_message_discriminants:?}");
                }
                match client_message_discriminants {
                    ClientMessageDiscriminants::Ping => respond_to_ping(),
                    ClientMessageDiscriminants::Version => respond_to_version(client_message),
                    ClientMessageDiscriminants::Init => respond_to_init(
                        client_message,
                        ws_connection_id,
                        &mut need_send_init_data_after_all,
                    ),
                    // ClientMessageDiscriminants::Login => {
                    //     respond_to_login(client_message, ws_connection_id).await
                    // }
                    ClientMessageDiscriminants::NeedInitData => {
                        respond_to_need_init_data(client_message, ws_connection_id).await
                    } // ============================================
                    // ============================================
                    // You have to customize:
                    // - here
                    ClientMessageDiscriminants::MoveCursor => {
                        respond_to_move_cursor(client_message, ws_connection_id).await
                    }
                    ClientMessageDiscriminants::AgentVersion => {
                        respond_to_agent_version(client_message)
                    }
                    ClientMessageDiscriminants::AgentFileChanged => {
                        respond_to_agent_file_changed(client_message).await
                    }
                    ClientMessageDiscriminants::AgentFileRemoved => {
                        respond_to_agent_file_removed(client_message).await
                    }
                    ClientMessageDiscriminants::AgentInit => {
                        respond_to_agent_init(client_message).await
                    } // ClientMessageDiscriminants::Refresh => respond_to_refresh(client_message), // ==========================================
                      // ==========================================
                }
            }
        } {
            send_message_to(message, send, ws_connection_id).await
        }
        post_process(need_send_init_data_after_all, ws_connection_id).await;
    }
}

// ============================================
// ============================================
// You have to customize:
// - here
//
pub async fn respond_to_move_cursor(
    client_message: ClientMessage,
    _ws_connection_id: WsConnectionId,
) -> Option<(ServerMessage, Send)> {
    if let ClientMessage::MoveCursor(MoveCursor { cursor, dance }) = client_message {
        // debug!("respond_to_move_cursor: cursor: {cursor:?}");
        *CURSOR.write().unwrap() = cursor;
        *DANCE.write().unwrap() = dance.clone();
        Some((
            ServerMessage::MoveCursor(MoveCursor { cursor, dance }),
            Send::AllButSelf,
        ))
    } else {
        unreachable!();
    }
}

// ==========================================
// ==========================================

pub fn respond_to_agent_version(
    client_message: ClientMessage,
) -> Option<(ServerMessage, common::Send)> {
    if let ClientMessage::AgentVersion(version_client) = client_message {
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
                "version_client: {version_client}, version_server: {version_server}"
            ))
        } else {
            let need_refresh = true;
            Ok(need_refresh)
        };
        Some((ServerMessage::AgentVersion(res), Send::ToSelf))
    } else {
        unreachable!();
    }
}

// use chrono::SecondsFormat;

pub async fn respond_to_agent_file_removed(
    client_message: ClientMessage,
) -> Option<(ServerMessage, common::Send)> {
    if let ClientMessage::AgentFileRemoved(file_name) = client_message {
        let mut did_update = false;
        process_file_did_remove(file_name, &mut did_update);
        if !did_update {
            None
        } else {
            Some((
                ServerMessage::update_heat_details_set(&HEAT_DETAILS.read().unwrap()),
                Send::AllButSelf,
            ))
        }
    } else {
        unreachable!();
    }
}

pub async fn respond_to_agent_file_changed(
    client_message: ClientMessage,
) -> Option<(ServerMessage, common::Send)> {
    if let ClientMessage::AgentFileChanged(res) = client_message {
        match res {
            Err(err) => {
                error!("{}:{}: {err}", file!(), line!());
                None
            }
            Ok(compressed_bytes) => {
                use flate2::read::GzDecoder;
                use std::io::prelude::*;
                let mut d = GzDecoder::new(&*compressed_bytes);
                let mut encoded = vec![];
                d.read_to_end(&mut encoded).unwrap();
                let AgentFileChanged {
                    file_name,
                    file_content,
                } = bincode::deserialize::<AgentFileChanged>(&encoded).unwrap();
                let mut did_update = false;
                process_file(file_name, file_content, &mut did_update);
                if !did_update {
                    None
                } else {
                    Some((
                        ServerMessage::update_heat_details_set(&HEAT_DETAILS.read().unwrap()),
                        Send::AllButSelf,
                    ))
                }
            }
        }
    } else {
        unreachable!();
    }
}

pub async fn respond_to_agent_init(
    client_message: ClientMessage,
) -> Option<(ServerMessage, common::Send)> {
    if let ClientMessage::AgentInit(res) = client_message {
        match res {
            Err(err) => {
                error!("{}:{}: {err}", file!(), line!());
                None
            }
            Ok(compressed_bytes) => {
                let mut did_update = false;
                let is_empty = HEAT_DETAILS.read().unwrap().is_empty();
                if !is_empty {
                    HEAT_DETAILS.write().unwrap().clear();
                    did_update = true;
                }
                use flate2::read::GzDecoder;
                use std::io::prelude::*;
                let mut d = GzDecoder::new(&*compressed_bytes);
                let mut encoded = vec![];
                d.read_to_end(&mut encoded).unwrap();
                let AgentInit { file_contents } =
                    bincode::deserialize::<AgentInit>(&encoded).unwrap();
                for (file_name, file_content) in file_contents {
                    process_file(file_name, file_content, &mut did_update);
                }
                if !did_update {
                    None
                } else {
                    Some((
                        ServerMessage::update_heat_details_set(&HEAT_DETAILS.read().unwrap()),
                        Send::AllButSelf,
                    ))
                }
            }
        }
    } else {
        unreachable!();
    }
}

fn get_file_name_category_heats_from_file_name(
    file_name: String,
) -> Option<(String, String, Vec<i16>)> {
    let Some(file_name) = file_name.strip_suffix(".heats") else { 
        debug!("{}:{}: {file_name:?}", file!(), line!());
        return None;
    };
    let mut ss = file_name
        .splitn(2, '.')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>();
    let Some(category) = ss.pop() else {
        debug!("{}:{}: {ss:?}, {file_name:?}", file!(), line!());
        return None;
    };
    let category = category.trim();
    if category.is_empty() {
        debug!("{}:{}: {ss:?}, {file_name:?}", file!(), line!());
        return None;
    }
    lazy_static::lazy_static! {
        pub static ref RE: regex::Regex = regex::Regex::new(r#"\D+"#).unwrap();
    }
    let Some(s) = ss.pop() else {
        debug!("{}:{}: {ss:?}, {file_name:?}", file!(), line!());
        return None;
    };
    let heats = RE
        .split(s)
        .filter_map(|s| s.parse::<i16>().ok())
        .collect::<Vec<_>>();
    if heats.is_empty() {
        debug!("{}:{}: {s}, {file_name:?}", file!(), line!());
        return None;
    }
    Some((file_name.to_owned(), category.to_owned(), heats))
}

fn process_file_did_remove(file_name: String, did_update: &mut bool) {
    if let Some((_file_name, category, heats)) =
        get_file_name_category_heats_from_file_name(file_name)
    {
        for heat in heats {
            let key = HeatDetailsKey {
                heat,
                category: category.to_owned(),
            };
            if HEAT_DETAILS.write().unwrap().remove(&key).is_some() {
                *did_update = true;
            }
        }
    }
}

fn process_file(file_name: String, file_content: Vec<u8>, did_update: &mut bool) {
    if let Some((file_name, category, heats)) =
        get_file_name_category_heats_from_file_name(file_name)
    {
        let (s, file_content) = match String::from_utf8(file_content.clone()) {
            Ok(s) => (Some(s), file_content),
            Err(_) => {
                let (cow, _encoding_used, _had_errors) =
                    encoding_rs::WINDOWS_1251.decode(&file_content);
                let file_content = cow.into_owned().as_bytes().to_vec();
                let s = match String::from_utf8(file_content.clone()) {
                    Ok(s) => Some(s),
                    Err(err) => {
                        error!("{}:{}: {file_name:?}: {err}", file!(), line!());
                        None
                    }
                };
                (s, file_content)
            }
        };
        let Some(s) = s else {
            debug!("{}:{}: file_content.len():{:?}", file!(), line!(),file_content.len());
            return;
        };
        let mut value = vec![];
        for s in s.lines() {
            let mut ss = s
                .splitn(3, ':')
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .collect::<Vec<_>>();
            if ss.len() < 2 {
                error!("{}:{}: s: {s:?}, ss: {ss:?}", file!(), line!());
                continue;
            }
            let Some(couples) = ss.pop() else {
                error!("{}:{}: s: {s:?}, ss: {ss:?}", file!(), line!());
                continue;
            };
            let couples = couples
                .split('|')
                .filter_map(|s| s.parse::<u16>().ok())
                .collect::<Vec<_>>();
            if couples.is_empty() {
                error!("{}:{}: s: {s:?}, ss: {ss:?}", file!(), line!());
                continue;
            }
            let Some(ord) = ss.pop() else { continue };
            let ord = if let Ok(ord) = ord.parse::<u8>() {
                ord
            } else {
                error!("{}:{}: s: {s:?}, ss: {ss:?}", file!(), line!());
                continue;
            };
            let dance = ss
                .pop()
                .and_then(|s| (!s.is_empty()).then_some(s.to_owned()));
            let item = HeatDetailsValueItem {
                dance,
                ord,
                couples,
            };
            value.push(item);
        }
        for heat in heats {
            let key = HeatDetailsKey {
                heat,
                category: category.to_owned(),
            };
            HEAT_DETAILS.write().unwrap().insert(key, value.clone());
            *did_update = true;
        }
    }
}

use std::collections::HashMap;
use std::sync::RwLock;
lazy_static::lazy_static! {
    pub static ref HEAT_DETAILS: RwLock<HeatDetails> = RwLock::new(HashMap::new());
    pub static ref CURSOR: RwLock<Option<i16>> = RwLock::new(None);
    pub static ref DANCE: RwLock<Option<String>> = RwLock::new(None);
}

// ==========================================
// ==========================================
