#[allow(unused_macros)]
macro_rules! ip(
    ($real_ip:expr, $forwarded_for:expr) => {
        if $real_ip.is_empty() {
            if $forwarded_for.is_empty() {
                "no IP".to_owned()
            } else {
                $forwarded_for.clone()
            }
        } else if $forwarded_for.is_empty() || $forwarded_for == $real_ip{
            $real_ip.clone()
        } else {
            format!("{}({})", $real_ip, $forwarded_for)
        }
    };
);

#[allow(unused_macros)]
macro_rules! send_receive(
    ($message:expr => $pat:pat, $real_ip:ident, $forwarded_for:ident) => {{
        let message = $message;
        let message_kind = super::RequestMessageKind::from(&message);
        let (tx, mut rx) =
            tokio::sync::mpsc::channel::<super::ResponseMessage>(1);
        if let Err(err) = (*TX.write().unwrap())
            .as_mut()
            .unwrap()
            .try_send((tx, message))
        {
            return Err(warp::reject::custom(error::Error::Anyhow(anyhow!(
                "{}: failed to send request {:?}: {}",
                ip!($real_ip, $forwarded_for),
                message_kind, err
            ))));
        }
        if let Some(response) = rx.recv().await
        {
            if matches!(response, $pat) {
                response
            } else {
                return Err(warp::reject::custom(error::Error::Anyhow(anyhow!(
                    "{}: got {} for {}",
                    ip!($real_ip, $forwarded_for),
                    ResponseMessageKind::from(&response),
                    message_kind
                ))));
            }
        } else {
            return Err(warp::reject::custom(error::Error::Anyhow(anyhow!(
                "{}: failed to recieve response for {}",
                ip!($real_ip, $forwarded_for),
                message_kind
            ))));
        }
    }};
    ($message:expr => $pat:pat) => {{
        let message = $message;
        let message_kind = RequestMessageKind::from(&message);
        let (tx, mut rx) =
            tokio::sync::mpsc::channel::<ResponseMessage>(1);
        if let Err(err) = (*TX.write().unwrap())
            .as_mut()
            .unwrap()
            .try_send((tx, message))
        {
            return Err(warp::reject::custom(error::Error::Anyhow(anyhow!(
                "failed to send request {:?}: {}",
                message_kind, err
            ))));
        }
        if let Some(response) = rx.recv().await
        {
            if matches!(response, $pat) {
                response
            } else {
                return Err(warp::reject::custom(error::Error::Anyhow(anyhow!(
                    "got {} for {}",
                    ResponseMessageKind::from(&response),
                    message_kind
                ))));
            }
        } else {
            return Err(warp::reject::custom(error::Error::Anyhow(anyhow!(
                "failed to recieve response for {}",
                message_kind
            ))));
        }
    }};
);
