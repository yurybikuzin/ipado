use super::*;
use thiserror::Error;
use warp::http::StatusCode;

#[derive(Error, Debug, strum::EnumDiscriminants)]
#[strum_discriminants(name(ErrorKind))]
pub enum Error {
    #[error("{0}")]
    Anyhow(anyhow::Error),
    #[error("{0}")]
    BadRequest(anyhow::Error),
    #[error("{0}")]
    NotFound(anyhow::Error),
    // #[error("{0:?}: доступ запрещен")]
    // ForbiddenFor(Option<login_export::AuthContact>),
}
impl warp::reject::Reject for Error {}

impl Clone for Error {
    fn clone(&self) -> Self {
        match self {
            Self::Anyhow(err) => Self::Anyhow(anyhow!("{err}")),
            Self::BadRequest(err) => Self::BadRequest(anyhow!("{err}")),
            Self::NotFound(err) => Self::NotFound(anyhow!("{err}")),
            // Self::ForbiddenFor(contact) => Self::ForbiddenFor(contact.clone()),
        }
    }
}

common_macros2::impl_from!(&Error => StatusCode, from,
    ErrorKind::from(from).into()
);

common_macros2::impl_from!(ErrorKind => StatusCode, from: From,
    match from {
        From::Anyhow => Self::INTERNAL_SERVER_ERROR,
        From::BadRequest => Self::BAD_REQUEST,
        From::NotFound => Self::NOT_FOUND,
        // From::ForbiddenFor => Self::FORBIDDEN,
    }
);
