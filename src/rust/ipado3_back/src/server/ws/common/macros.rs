macro_rules! send_receive(
    ($message:expr => $pat:pat => $($code:tt)+) => {{
        let message = $message;
        let message_kind = RequestMessageKind::from(&message);
        let (tx, mut rx) =
            tokio::sync::mpsc::channel::<ResponseMessage>(1);
        let ret = (*crate::server::common::TX.write().unwrap())
            .as_mut()
            .unwrap()
            .try_send((tx, message));
        if let Err(err) = ret
        {
            Err(anyhow!(
                "failed to send request {:?}: {}",
                message_kind, err
            ))
        } else if let Some(response) = rx.recv().await {
            #[allow(unreachable_patterns)]
            match response {
                $pat => Ok($($code)+),
                _ => Err(anyhow!(
                    "got {} for {}",
                    ResponseMessageKind::from(&response),
                    message_kind
                ))
            }
        } else {
            Err(anyhow!(
                "failed to recieve response for {}",
                message_kind
            ))
        }
    }};
);
