use super::*;

use web_sys::Url;

use serde::{Deserialize, Serialize};
// use strum::IntoEnumIterator;
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Route {
    Guest(GuestRoute),
    User(UserRoute),
    // User(UserRoute),
    // pub context: Context,
    // pub modal_stack: Vec<Modal>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, strum::EnumIter, strum::Display)]
pub enum GuestRoute {
    Tablo,
    Heat,
    // Main,
    // Board,
    // Mobile,
    // #[strum(serialize = "Пары")]
    // Couples,
    // #[strum(serialize = "Расписание")]
    // Schedule,
    // CoupleHeatLists {
    //     number: i16,
    // },
    // #[strum(serialize = "Заходы")]
    // Heats,
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    strum::FromRepr,
    strum::EnumIter,
    strum::Display,
)]
#[repr(u8)]
pub enum UserRoute {
    Main,
    // #[strum(serialize = "Пары")]
    // Couplelar,
    // #[strum(serialize = "Расписание")]
    // Schedule,
}

impl Default for Route {
    fn default() -> Self {
        // Create the Route based on the current URL
        Self::from_url(&routing::url().lock_ref()) //.unwrap_or_else(|| Self::default_value())
    }
}

impl Route {
    //     fn pop_modal(self) -> Self {
    //         let Self {
    //             context,
    //             mut modal_stack,
    //         } = self;
    //         modal_stack.pop();
    //         Self {
    //             context,
    //             modal_stack,
    //         }
    //     }
    //     fn push_modal(self, modal: Modal) -> Self {
    //         let Self {
    //             context,
    //             mut modal_stack,
    //         } = self;
    //         modal_stack.push(modal);
    //         Self {
    //             context,
    //             modal_stack,
    //         }
    //     }
    //     fn update_modal(self, modal: Modal) -> Self {
    //         let Self {
    //             context,
    //             mut modal_stack,
    //         } = self;
    //         modal_stack.pop();
    //         modal_stack.push(modal);
    //         Self {
    //             context,
    //             modal_stack,
    //         }
    //     }
    // fn default_value() -> Self {
    //     Self {
    //         // context: Context::Club,
    //         // modal_stack: vec![],
    //     }
    // }
    //     fn context(context: Context) -> Self {
    //         Self {
    //             context,
    //             modal_stack: vec![],
    //         }
    //     }
    //     // This could use more advanced URL parsing, but it isn't needed
    // pub fn from_url(url: &str) -> Self {
    //     let url = Url::new(url).unwrap();
    //     let hash = url.hash();
    //     let hash = hash.as_str();
    //     hash.starts_with("#/")
    //         .then(|| {
    //             base64::decode(&hash[2..])
    //                 .ok()
    //                 .and_then(|buf| rmp_serde::from_slice::<Route>(&buf).ok())
    //         })
    //         .and_then(|ret| ret)
    //         .unwrap_or_else(|| {
    //             if url.pathname().ends_with("/admin/") {
    //                 Route::User(UserRoute::Couplelar)
    //             } else {
    //                 Route::Guest(GuestRoute::Main)
    //             }
    //         })
    // }
    pub fn from_url(url: &str) -> Self {
        let url = Url::new(url).unwrap();
        if url.pathname().ends_with("/admin/") {
            Self::User(UserRoute::Main)
        } else {
            debug!("url.hash(): {}", url.hash().as_str());
            if url.hash().as_str().starts_with("#/heat") {
                Some(Self::Guest(GuestRoute::Heat))
                // Self::from_url_hash(url.hash().as_str())
            } else {
                None
            }
            .unwrap_or(Route::Guest(GuestRoute::Tablo))
        }
    }
    // pub fn from_url_hash(url_hash: &str) -> Option<Self> {
    //     URL_SAFE_BASE64
    //         .decode(&url_hash[2..])
    //         .ok()
    //         .and_then(|buf| rmp_serde::from_slice::<Route>(&buf).ok())
    //         .or_else(|| {
    //             // for backward compatibility
    //             BASE64
    //                 .decode(&url_hash[2..])
    //                 .ok()
    //                 .and_then(|buf| rmp_serde::from_slice::<Route>(&buf).ok())
    //         })
    // }
    // pub fn to_url(&self) -> String {
    //     let ret = format!(
    //         "#/{}",
    //         URL_SAFE_BASE64.encode(rmp_serde::encode::to_vec(&self).unwrap()) // base64::encode(&rmp_serde::encode::to_vec(&self).unwrap())
    //     );
    //     ret
    // }
}

// use base64::{alphabet, engine, Engine as _};
// pub static BASE64: Lazy<Arc<engine::general_purpose::GeneralPurpose>> = Lazy::new(|| {
//     let config = engine::GeneralPurposeConfig::new()
//         .with_decode_allow_trailing_bits(true)
//         .with_encode_padding(false)
//         .with_decode_padding_mode(engine::DecodePaddingMode::Indifferent);
//     // Arc::new(engine::GeneralPurpose::new(&alphabet::URL_SAFE, config))
//     Arc::new(engine::GeneralPurpose::new(&alphabet::STANDARD, config))
// });
//
// pub static URL_SAFE_BASE64: Lazy<Arc<engine::general_purpose::GeneralPurpose>> = Lazy::new(|| {
//     let config = engine::GeneralPurposeConfig::new()
//         .with_decode_allow_trailing_bits(true)
//         .with_encode_padding(false)
//         .with_decode_padding_mode(engine::DecodePaddingMode::Indifferent);
//     Arc::new(engine::GeneralPurpose::new(&alphabet::URL_SAFE, config))
// });
