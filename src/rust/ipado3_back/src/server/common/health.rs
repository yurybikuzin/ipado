use super::*;

pub fn api() -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Copy {
    warp::get()
        .and(warp::path("health"))
        .and(warp::path::end())
        .and_then(handle)
}

async fn handle() -> std::result::Result<impl Reply, Rejection> {
    Ok(format!(
        "{} {} {}\n",
        *OP_MODE.read().unwrap(),
        PKG_NAME,
        PKG_VERSION,
    ))
}
