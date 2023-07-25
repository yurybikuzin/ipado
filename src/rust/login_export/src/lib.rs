use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize, Hash)]
#[serde(rename_all = "camelCase")]
pub enum GetInfoRet {
    Google(GetGoogleInfoRet),
    Mailru(GetMailruInfoRet),
    Yandex(GetYandexInfoRet),
    Tinkoff(GetTinkoffInfoRet),
}

// pub type GetGoogleInfoRet = google_jwt_verify::IdPayload;
pub type GetGoogleInfoRet = IdPayload;

#[derive(Debug, Clone, Serialize, Deserialize, Hash)]
pub struct IdPayload {
    pub iss: Option<String>,
    pub nbf: Option<u64>,
    pub aud: Option<String>,
    pub sub: Option<String>,
    pub nonce: Option<String>, //added by yb
    pub hd: Option<String>,
    pub email: Option<String>,
    pub email_verified: Option<bool>,
    pub azp: Option<String>,
    pub name: Option<String>,
    pub picture: Option<String>,
    pub given_name: Option<String>,
    pub family_name: Option<String>,
    pub iat: Option<u64>,
    pub exp: Option<u64>,
    pub jti: Option<String>,
}
#[derive(Debug, Serialize, Deserialize, Hash)]
pub struct GetMailruInfoRet {
    pub nickname: Option<String>,
    pub client_id: Option<String>,
    pub id: Option<String>,
    pub image: Option<String>,
    pub first_name: Option<String>,
    pub email: Option<String>,
    pub locale: Option<String>,
    pub name: Option<String>,
    pub last_name: Option<String>,
    pub birthday: Option<String>,
    pub gender: Option<String>,
}
#[derive(Debug, Serialize, Deserialize, Hash)]
pub struct GetYandexInfoRet {
    pub id: Option<String>,
    pub login: Option<String>,
    pub client_id: Option<String>,
    pub default_email: Option<String>,
    pub emails: Option<Vec<String>>,
    pub default_avatar_id: Option<String>,
    pub is_avatar_empty: Option<bool>,
    pub psuid: Option<String>,
}
#[derive(Debug, Serialize, Deserialize, Hash)]
pub struct GetTinkoffInfoRet {
    pub phone_number_verified: Option<bool>,
    pub email: Option<String>,
    pub family_name: Option<String>,
    pub birthdate: Option<String>,
    pub middle_name: Option<String>,
    pub sub: Option<String>,
    pub given_name: Option<String>,
    pub gender: Option<String>,
    pub name: Option<String>,
    pub phone_number: Option<String>,
    pub email_verified: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct TokenPayload {
    pub token: String,
    pub fingerprint: String,
    pub secret_key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthRet {
    pub contact: AuthContact,
    pub details: AuthRetDetails,
}
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AuthRetDetails {
    pub nickname: Option<String>,
    pub name: Option<String>,
    pub given_name: Option<String>,
    pub middle_name: Option<String>,
    pub family_name: Option<String>,
    pub birthday: Option<String>,
    pub phone_number: Option<String>,
    pub email: Option<String>,
    pub emails: Option<Vec<String>>,
    pub gender: Option<String>,
    pub picture: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AclContact<'a> {
    Email(&'a str),
    PhoneNumber(&'a str),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum AuthContact {
    Email(String),
    PhoneNumber(String),
}
common_macros2::impl_display!(
    AuthContact,
    self,
    "{}",
    match &self {
        Self::Email(s) => s,
        Self::PhoneNumber(s) => s,
    }
);

use std::sync::Arc;
//
pub struct AclRoles(pub Vec<Arc<dyn AclRole + Sync + Send>>);
impl AclRoles {
    pub fn new(roles: Vec<Arc<dyn AclRole + Sync + Send>>) -> Self {
        Self(roles)
    }
    pub fn can(
        &self,
        action: &Box<dyn Any>,
        resource: Option<&Box<dyn Any>>,
        attr: Option<&Box<dyn Any>>,
    ) -> bool {
        let mut ret: Option<bool> = None;
        for role in self.0.iter() {
            match role.can(action, resource, attr) {
                None => continue,
                value @ Some(true) => {
                    ret = value;
                }
                value @ Some(false) => {
                    ret = value;
                    break;
                }
            }
        }
        ret.unwrap_or(false)
    }
}
pub trait AclRole {
    fn can(
        &self,
        action: &Box<dyn Any>,
        resource: Option<&Box<dyn Any>>,
        attr: Option<&Box<dyn Any>>,
    ) -> Option<bool>;
    fn as_any(&self) -> &dyn Any;
}
pub type AclAuthResult = Result<AuthRet, Option<AuthContact>>;
use op_mode::OpMode;
use std::any::Any;
pub trait Acl {
    fn get_roles_for(&self, contact: &AuthContact, op_mode: OpMode) -> Option<AclRoles>;
    fn auth(&self, info: Box<GetInfoRet>, op_mode: OpMode) -> AclAuthResult {
        match *info {
            GetInfoRet::Google(GetGoogleInfoRet {
                name,
                given_name,
                family_name,
                email,
                email_verified,
                picture,
                ..
            }) => email_verified
                .and_then(|email_verified| {
                    email_verified
                        .then(|| {
                            email.clone().and_then(|email| {
                                let contact = AuthContact::Email(email);
                                self.get_roles_for(&contact, op_mode)
                                    .map(|_| Ok(contact.clone()))
                                    .or(Some(Err(contact)))
                            })
                        })
                        .and_then(|ret| ret)
                })
                .map(|res| {
                    res.map_err(Some).map(|contact| AuthRet {
                        details: AuthRetDetails {
                            name,
                            given_name,
                            family_name,
                            email,
                            picture,
                            ..AuthRetDetails::default()
                        },
                        contact,
                    })
                }),
            GetInfoRet::Mailru(GetMailruInfoRet {
                nickname,
                email,
                name,
                first_name: given_name,
                last_name: family_name,
                birthday,
                image: picture,
                gender,
                ..
            }) =>
            //
            {
                email
                    .clone()
                    .and_then(|email| {
                        let contact = AuthContact::Email(email);
                        self.get_roles_for(&contact, op_mode)
                            .map(|_| Ok(contact.clone()))
                            .or(Some(Err(contact)))
                    })
                    //
                    .map(|res| {
                        res.map_err(Some).map(|contact| AuthRet {
                            details: AuthRetDetails {
                                nickname,
                                name,
                                given_name,
                                family_name,
                                birthday,
                                email,
                                gender,
                                picture,
                                ..AuthRetDetails::default()
                            },
                            contact,
                        })
                    })
            }
            GetInfoRet::Yandex(GetYandexInfoRet {
                default_email: email,
                emails,
                default_avatar_id,
                ..
            }) => {
                let ret = email.and_then(|email| {
                    let contact = AuthContact::Email(email);
                    self.get_roles_for(&contact, op_mode)
                        .map(|_| Ok(contact.clone()))
                        .or(Some(Err(contact)))
                });
                let ret = if matches!(ret, Some(Ok(_))) {
                    ret
                } else {
                    emails.as_ref().and_then(|emails| {
                        emails
                            .iter()
                            .filter_map(|email| {
                                let contact = AuthContact::Email(email.clone());
                                self.get_roles_for(&contact, op_mode)
                                    .map(|_| Ok(contact.clone()))
                            })
                            .next()
                            .or(ret)
                    })
                };
                ret.map(|res| {
                    res.map_err(Some).map(|contact| {
                        let email = if let AuthContact::Email(email) = contact.clone() {
                            email
                        } else {
                            unreachable!()
                        };
                        AuthRet {
                            details: AuthRetDetails {
                                emails: {
                                    emails
                                        .map(|emails| {
                                            emails
                                                .iter()
                                                .cloned()
                                                .chain(vec![email.clone()].into_iter())
                                                .filter(|i| i != &email)
                                                .collect::<std::collections::HashSet<_>>()
                                                .into_iter()
                                                .collect::<Vec<_>>()
                                        })
                                        .and_then(
                                            |ret| if ret.is_empty() { None } else { Some(ret) },
                                        )
                                },
                                email: Some(email),
                                picture: default_avatar_id.map(|default_avatar_id| {
                                    format!(
                                        "https://avatars.mds.yandex.net/get-yapic/{}",
                                        default_avatar_id
                                    )
                                }),
                                ..AuthRetDetails::default()
                            },
                            contact,
                        }
                    })
                })
            }
            GetInfoRet::Tinkoff(GetTinkoffInfoRet {
                email,
                email_verified,
                phone_number,
                phone_number_verified,
                family_name,
                birthdate: birthday,
                middle_name,
                given_name,
                gender,
                name,
                ..
            }) => {
                let ret = phone_number_verified
                    .as_ref()
                    .and_then(|phone_number_verified| {
                        phone_number_verified
                            .then(|| {
                                phone_number.as_ref().and_then(|phone_number| {
                                    let contact =
                                        AuthContact::PhoneNumber(phone_number.to_string());
                                    self.get_roles_for(&contact, op_mode)
                                        .map(|_| Ok(contact.clone()))
                                        .or(Some(Err(contact)))
                                })
                            })
                            .and_then(|ret| ret)
                    });
                let ret = if matches!(ret, Some(Ok(_))) {
                    ret
                } else {
                    email_verified
                        .as_ref()
                        .and_then(|email_verified| {
                            email_verified
                                .then(|| email.as_ref().map(|email| email.to_string()))
                                .and_then(|ret| ret)
                        })
                        .and_then(|email| {
                            let contact = AuthContact::Email(email);
                            self.get_roles_for(&contact, op_mode)
                                .map(|_| Ok(contact.clone()))
                        })
                        .or(ret)
                };
                ret.map(|res| {
                    res.map_err(Some).map(|contact| AuthRet {
                        details: AuthRetDetails {
                            name,
                            given_name,
                            middle_name,
                            family_name,
                            birthday,
                            phone_number: phone_number_verified.and_then(|phone_number_verified| {
                                phone_number_verified
                                    .then_some(phone_number)
                                    .and_then(|phone_number| phone_number)
                            }),
                            email: email_verified
                                .and_then(|email_verified| email_verified.then_some(email))
                                .and_then(|email| email),
                            gender,
                            ..AuthRetDetails::default()
                        },
                        contact,
                    })
                })
            }
        }
        .unwrap_or(Err(None))
    }
}
