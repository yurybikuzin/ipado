#[allow(unused_imports)]
use anyhow::{anyhow, bail, Context, Error, Result};

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

use chrono::NaiveTime;
use flate2::{read::GzDecoder, write::GzEncoder, Compression};
pub use login_export::*;
pub use op_mode::OpMode;
use serde::{Deserialize, Serialize};
use std::io::prelude::*;
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize, strum::EnumDiscriminants)]
pub enum ClientMessage {
    Ping,
    Version(semver::Version),
    Init(Vec<u8>),
    NeedInitData { key: InitDataKey, refresh: bool },
    //
    MoveCursor(MoveCursor),
    //
    AgentVersion(semver::Version),
    AgentFileChanged(std::result::Result<Vec<u8>, String>),
    AgentFileRemoved(String),
    AgentInit(std::result::Result<Vec<u8>, String>),
}

#[derive(Debug, Clone, Serialize, Deserialize, strum::EnumDiscriminants)]
pub enum ServerMessage {
    Pong,
    Version(std::result::Result<NeedRefresh, String>),
    Login(Option<Vec<u8>>),
    InitData(std::result::Result<Vec<u8>, String>),
    UpdateHeatDetails(Vec<u8>),
    MoveCursor(MoveCursor),
    //
    AgentVersion(std::result::Result<NeedRefresh, String>),
}

#[derive(Debug)]
pub enum AgentFile {
    Removed(String),
    Changed(AgentFileChanged),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentFileChanged {
    pub file_name: String,
    pub file_content: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentInit {
    pub file_contents: std::collections::HashMap<String, Vec<u8>>,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct HeatDetailsKey {
    pub heat: i16,
    pub category: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HeatDetailsValueItem {
    pub dance: Option<String>,
    pub ord: u8,
    pub couples: Vec<u16>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoveCursor {
    pub cursor: Option<i16>,
    pub dance: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub struct InitDataKey {}
pub type NeedRefresh = bool;

pub type Email = String;
pub type UserId = i16;

use std::collections::HashMap;
pub type HeatDetails = HashMap<HeatDetailsKey, Vec<HeatDetailsValueItem>>;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InitData {
    pub heatlar: Vec<Heat>,
    pub cursor: Option<i16>,
    pub dance: Option<String>,
    pub heat_details: HeatDetails,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Heat {
    pub number: i16,
    pub time: Time,
    pub category_tourlar: Vec<CategoryTour>,
    pub floor: Option<String>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CategoryTour {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub time: Option<Time>,
    pub category: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub tour: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub program: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub quantity: Option<i16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub dances: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub heats: Option<i16>,
}
common_macros2::impl_display!(CategoryTour, self, f, {
    if let Some(time) = &self.time {
        write!(f, "{time}")?;
    }
    if let Some(tour) = &self.tour {
        if self.time.is_some() {
            write!(f, ", ")?;
        }
        write!(f, "{tour}")?;
    }
    if !self.category.is_empty() {
        if self.time.is_some() || self.tour.is_some() {
            write!(f, ", ")?;
        }
        let _ = write!(f, "{}", self.category);
        if let Some(dances) = &self
            .dances
            .as_ref()
            .and_then(|dances| dances.contains(' ').then_some(dances))
        {
            write!(f, " ({dances})")?;
        }
        if self.tour.is_some() && self.dances.is_some() {
            if let Some(quantity) = self.quantity {
                write!(
                    f,
                    ": {quantity} {}",
                    common_macros2::plural!(quantity, 1 "пара", 2 "пары", 5 "пар")
                )?;
                if let Some(heats) = self.heats {
                    write!(
                        f,
                        " ({heats} {})",
                        common_macros2::plural!(heats, 1 "заход", 2 "захода", 5 "заходов")
                    )?;
                }
            }
        }
    }
    Ok(())
});

use serde_with::{serde_as, DurationSeconds};
#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Time {
    pub start_at: NaiveTime,
    #[serde_as(as = "DurationSeconds<u64>")]
    pub duration: Duration,
}

common_macros2::impl_display!(
    Time,
    self,
    "{} → {}",
    self.start_at.format("%H:%M").to_string(),
    (self.start_at + chrono::Duration::from_std(self.duration).unwrap())
        .format("%H:%M")
        .to_string()
);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    pub protocol: String,
    pub host: String,
    pub port: Option<u16>,
    pub pathname: String,
    pub search: Option<String>,
    pub hash: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientMessageInit {
    pub location: Location,
    pub auth: Option<AuthRet>,
    pub key: InitDataKey,
}

impl ClientMessage {
    pub fn init_set(params: &ClientMessageInit) -> Self {
        ClientMessage::Init(compress_bincoded(&params).unwrap())
    }
    pub fn init_get(self) -> Option<ClientMessageInit> {
        if let ClientMessage::Init(compressed_bytes) = self {
            decompress_bincoded::<ClientMessageInit>(compressed_bytes).ok()
        } else {
            None
        }
    }
    pub fn encoded(&self) -> Vec<u8> {
        bincode::serialize(&self).unwrap()
    }
    pub fn from_encoded(value: &[u8]) -> Result<Self> {
        debug!("{}:{}: encoded.len: {}", file!(), line!(), value.len());
        bincode::deserialize(value).map_err(|err| anyhow!("{}:{}: {err}", file!(), line!()))
    }
}

impl ServerMessage {
    pub fn login_set(res: Option<login_export::AclAuthResult>) -> Self {
        Self::Login(match res {
            None | Some(Err(_)) => None,
            Some(Ok(auth)) => Some(compress_bincoded(&auth).unwrap()),
        })
    }
    pub fn login_get(self) -> Option<AuthRet> {
        if let Self::Login(Some(compressed_bytes)) = self {
            decompress_bincoded::<login_export::AuthRet>(compressed_bytes).ok()
        } else {
            None
        }
    }
    pub fn init_data_set(init_data: &InitData) -> Self {
        Self::InitData(compress_yamled(&init_data))
    }
    pub fn init_data_get(self) -> Option<std::result::Result<InitData, String>> {
        if let Self::InitData(res) = self {
            Some(match res {
                Ok(compressed_bytes) => {
                    decompress_yamled::<InitData>(compressed_bytes).map_err(|err| format!("{err}"))
                }
                Err(err) => Err(err),
            })
        } else {
            None
        }
    }

    pub fn update_heat_details_set(heat_details: &HeatDetails) -> Self {
        Self::UpdateHeatDetails(compress_yamled(heat_details).unwrap())
    }
    pub fn update_heat_details_get(self) -> Option<HeatDetails> {
        if let Self::UpdateHeatDetails(compressed_bytes) = self {
            decompress_yamled::<HeatDetails>(compressed_bytes)
                .map_err(|err| format!("{err}"))
                .ok()
        } else {
            None
        }
    }

    pub fn encoded(&self) -> Vec<u8> {
        bincode::serialize(&self).unwrap()
    }
    pub fn from_encoded(value: &[u8]) -> Result<Self> {
        bincode::deserialize(value).map_err(|err| anyhow!("{}:{}: {err}", file!(), line!()))
    }
}

fn compress_bincoded<S: Serialize>(data: &S) -> std::result::Result<Vec<u8>, String> {
    let mut e = GzEncoder::new(Vec::new(), Compression::default());
    bincode::serialize(&data)
        .map_err(|err| format!("{err}"))
        .and_then(|encoded| {
            e.write_all(&encoded)
                .map_err(|err| format!("{err}"))
                .and_then(|_| e.finish().map_err(|err| format!("{err}")))
        })
}

fn decompress_bincoded<T: serde::de::DeserializeOwned>(
    compressed_bytes: Vec<u8>,
) -> std::result::Result<T, Box<bincode::ErrorKind>> {
    let mut d = GzDecoder::new(&*compressed_bytes);
    let mut encoded = vec![];
    d.read_to_end(&mut encoded).unwrap();
    bincode::deserialize::<T>(&encoded)
}

fn compress_yamled<S: Serialize>(data: &S) -> std::result::Result<Vec<u8>, String> {
    let mut e = GzEncoder::new(Vec::new(), Compression::default());
    serde_yaml::to_string(&data)
        .map_err(|err| format!("{err}"))
        .and_then(|yamled| {
            e.write_all(yamled.as_bytes())
                .map_err(|err| format!("{err}"))
                .and_then(|_| e.finish().map_err(|err| format!("{err}")))
        })
}

fn decompress_yamled<T: serde::de::DeserializeOwned>(compressed_bytes: Vec<u8>) -> Result<T> {
    let mut d = GzDecoder::new(&*compressed_bytes);
    let mut gzipped = vec![];
    d.read_to_end(&mut gzipped).unwrap();
    let s = std::str::from_utf8(&gzipped).map_err(|err| anyhow!(err))?;
    serde_yaml::from_str::<T>(s).map_err(|err| anyhow!(err))
}

// ====================== ACL ========================

use login_export::{AclContact, AclRoles, AuthContact, AuthRet};
use std::any::Any;
//
pub struct AclRoleManager {}
impl login_export::AclRole for AclRoleManager {
    fn can(
        &self,
        _action: &Box<dyn Any>,
        _resource: Option<&Box<dyn Any>>,
        _attr: Option<&Box<dyn Any>>,
    ) -> Option<bool> {
        None
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct Acl {}
use std::sync::Arc;
impl login_export::Acl for Acl {
    fn get_roles_for(&self, contact: &AuthContact, op_mode: OpMode) -> Option<AclRoles> {
        let contact = match contact {
            AuthContact::Email(ref email) => AclContact::Email(email),
            AuthContact::PhoneNumber(ref email) => AclContact::PhoneNumber(email),
        };
        match (contact, op_mode) {
            (AclContact::Email("yury.bikuzin@gmail.com"), _) => Some(login_export::AclRoles::new(
                vec![Arc::new(AclRoleManager {})],
            )),

            (_, _) => None,
        }
    }
}
