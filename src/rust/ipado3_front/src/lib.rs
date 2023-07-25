#[allow(unused_imports)]
use web_sys_utils::{debug, error, warn};

pub mod built_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}
use built_info::*;

#[allow(unused_imports)]
use dominator::{clone, events, html, link, routing, routing::go_to_url, with_node, Dom};

#[allow(unused_imports)]
use futures_signals::{
    map_ref,
    signal::{always, Mutable, Signal, SignalExt},
    signal_map::{MapDiff, MutableBTreeMap, SignalMapExt},
    signal_vec::{MutableVec, SignalVecExt, VecDiff},
};

#[allow(unused_imports)]
use web_sys::{window, ErrorEvent, HtmlElement, HtmlInputElement, MessageEvent, WebSocket};

use gloo_timers::callback::Timeout;
use once_cell::sync::Lazy;
use op_mode::OpMode;
use std::sync::Arc;
use std::sync::RwLock;
use wasm_bindgen::{prelude::*, JsCast};

mod common;
use common::*;

mod render;

#[wasm_bindgen(start)]
pub async fn main_js() -> Result<(), JsValue> {
    debug!(@ "{PKG_NAME} {PKG_VERSION}");
    common::main_js_helper()
}

// ==================================================
// ==================================================

fn process_server_message(server_message: ServerMessage) {
    let server_message_discriminants = ServerMessageDiscriminants::from(&server_message);
    match server_message_discriminants {
        ServerMessageDiscriminants::Pong => common::respond_to_pong(server_message),
        ServerMessageDiscriminants::Version => common::respond_to_version(server_message),
        ServerMessageDiscriminants::Login => common::respond_to_login(server_message),
        ServerMessageDiscriminants::InitData => common::respond_to_init_data(server_message),
        // ==================================================
        // ==================================================
        // You have to customize:
        // - here
        ServerMessageDiscriminants::MoveCursor => respond_to_move_cursor(server_message),
        ServerMessageDiscriminants::AgentVersion => {
            warn!(@ "server_message_discriminants: {server_message_discriminants:?}");
        }
        ServerMessageDiscriminants::UpdateHeatDetails => {
            respond_to_update_heat_details(server_message)
        } // ==================================================
          // ==================================================
    }
}

// ==================================================
// ==================================================
// You have to customize:
// - here

fn respond_to_update_heat_details(server_message: ServerMessage) {
    if let Some(heat_details) = server_message.clone().update_heat_details_get() {
        APP.data.heat_details.lock_mut().replace_cloned(
            heat_details
                .into_iter()
                .filter_map(|(key, value)| {
                    HeatDetailsValue::try_from(value)
                        .ok()
                        .map(|value| (key, value))
                })
                .map(|(key, value)| (key, Arc::new(value)))
                .collect(),
        );
    } else {
        error!(@ "unreachable: {server_message:?}");
    }
}

fn respond_to_move_cursor(server_message: ServerMessage) {
    if let ServerMessage::MoveCursor(MoveCursor { cursor, dance }) = server_message {
        *APP.data.cursor.lock_mut() = cursor;
        *APP.data.dance.lock_mut() = dance;
        render::schedule::scroll_to_cursor();
    } else {
        error!(@ "unreachable: {server_message:?}");
    }
}
use ipado3_common::*;

pub struct App {
    pub data: AppData,
    pub clock: Mutable<Clock>,
}

#[derive(Clone)]
pub enum AppMode {
    Guest,
    User(Arc<User>),
}

use std::collections::{BTreeMap, BTreeSet, HashMap};
#[derive(Debug)]
pub enum HeatDetailsValue {
    Simple {
        couples: Vec<u16>,
    },
    NonMixed {
        ords: BTreeSet<u8>,
        rows: Vec<HashMap<u8, u16>>,
    },
    Mixed {
        dances: Vec<String>,
        couples: BTreeMap<u16, HashMap<String, u8>>,
    },
}

use std::convert::TryFrom;
common_macros2::impl_try_from!(
    Vec<HeatDetailsValueItem> => HeatDetailsValue, String, from, {
        let ret = if from.len() == 1 {
            let HeatDetailsValueItem { couples, .. } = from.into_iter().next().unwrap();
            Ok(Self::Simple { couples })
        } else {
            let dance_count = from.iter().filter_map(|HeatDetailsValueItem { dance, .. }| dance.as_ref()).count();
            if dance_count == 0 {
                let mut rows: HashMap<usize, HashMap<u8, u16>> = HashMap::new();
                for HeatDetailsValueItem { ord, couples, .. } in from.into_iter() {
                    for (i, couple) in couples.into_iter().enumerate() {
                        common_macros2::entry!(rows, i
                        =>
                            and_modify |e| {
                                common_macros2::entry!(e, ord
                                =>
                                    and_modify |e| {
                                        *e = couple;
                                    }
                                    or_insert couple
                                )
                            }
                            or_insert HashMap::from([(ord, couple)])
                        );
                    }
                }
                let mut rows = rows.into_iter().collect::<Vec<_>>();
                rows.sort_by_key(|(i, _)| *i);

                let mut col_count = 0;
                for (_i, row) in rows.iter() {
                    if let Some(ord) = row.iter().map(|(ord, _couple)| ord).max() {
                        col_count = std::cmp::max(*ord, col_count)
                    }
                }

                let mut ords = BTreeSet::new();
                let rows = rows.into_iter().map(|(_, row)| {
                    for (ord, _) in row.iter() {
                        ords.insert(*ord);
                    }
                    row
                }).collect::<Vec<_>>();
                Ok(Self::NonMixed {
                    ords,
                    rows,
                })
            } else if dance_count == from.len(){
                let mut dances: Vec<String> = vec![];
                let mut couples: HashMap<u16, HashMap<String, u8>> = HashMap::new();

                for HeatDetailsValueItem { ord, dance, couples: item_couples } in from.into_iter() {
                    if let Some(dance) = dance {
                        if !dances.iter().any(|i| dance == i.as_str()) {
                            dances.push(dance.to_owned());
                        }
                        for couple in item_couples.into_iter() {
                            let dance = dance.clone();
                            common_macros2::entry!(couples, couple
                            =>
                                and_modify |e| {
                                    common_macros2::entry!(e, dance
                                    =>
                                        and_modify |e| {
                                            *e = ord;
                                        }
                                        or_insert ord
                                    );
                                }
                                or_insert HashMap::from([(dance, ord)])
                            );
                        }
                    }
                }
                let couples = couples.into_iter().collect::<BTreeMap<_, _>>();
                Ok(Self::Mixed {
                    dances,
                    couples,
                })
            } else {
                Err(format!("dance_count: {dance_count}, from.len(): {}", from.len()))
            }
        };
        ret
    }
);

#[derive(Default)]
pub struct AppData {
    pub is_alive_ws: Mutable<bool>,
    pub app_mode: Mutable<Option<AppMode>>,
    pub route: Mutable<Route>,
    pub is_refreshing: Mutable<bool>,

    pub heatlar: Mutable<Arc<Vec<Heat>>>,
    pub cursor: Mutable<Option<i16>>,
    pub dance: Mutable<Option<String>>,
    pub heat_details: MutableBTreeMap<HeatDetailsKey, Arc<HeatDetailsValue>>,
}

mod route;
use route::*;

#[derive(Clone, Copy)]
pub enum Clock {
    Even(i64),
    Odd(i64),
}

use gloo_timers::callback::Interval;

impl App {
    pub fn new_helper() -> Self {
        Interval::new(1_000, || {
            let value = *APP.clock.lock_ref();
            let timestamp = js_sys::Date::now() as i64 / 1000i64;
            match value {
                Clock::Odd(_) => APP.clock.set(Clock::Even(timestamp)),
                Clock::Even(_) => APP.clock.set(Clock::Odd(timestamp)),
            }
        })
        .forget();
        Self {
            clock: Mutable::new(Clock::Odd(js_sys::Date::now() as i64 / 1000i64)),
            data: AppData::default(),
        }
    }
    pub fn init_data_key() -> InitDataKey {
        InitDataKey {}
    }
    pub fn init(init_data: InitData) {
        *APP.data.heatlar.lock_mut() = Arc::new(init_data.heatlar);
        *APP.data.cursor.lock_mut() = init_data.cursor;
        *APP.data.dance.lock_mut() = init_data.dance;
        APP.data.heat_details.lock_mut().replace_cloned(
            init_data
                .heat_details
                .into_iter()
                .filter_map(|(key, value)| {
                    HeatDetailsValue::try_from(value)
                        .ok()
                        .map(|value| (key, value))
                })
                .map(|(key, value)| (key, Arc::new(value)))
                .collect(),
        );
        *APP.data.is_refreshing.lock_mut() = false;
        let is_none = APP.data.app_mode.lock_ref().is_none();
        if is_none {
            *APP.data.app_mode.lock_mut() = Some(AppMode::Guest);
            debug!("did set AppMode::Guest");
        }
        loaded()
    }
}
