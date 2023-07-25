use super::*;

pub type WsRead = WebSocketReadHalf;

pub type ConnectResult = Result<WsRead>;
pub async fn connect(url: &str) -> ConnectResult {
    match WebSocket::connect(url).await {
        Err(err) => {
            bail!("WebSocket::connect({url:?}): {err}")
        }
        Ok(mut ws) => {
            connect_helper(&mut ws).await?;
            let (read_half, write_half) = ws.split();
            debug!("{}:{}: did split", file!(), line!());
            *WS.write().await = Some(write_half);
            send_agent_init().await?;
            Ok(read_half)
        }
    }
}
pub async fn connect_helper(ws: &mut WebSocket) -> Result<()> {
    ws.send_binary(
        ClientMessage::AgentVersion(semver::Version::parse(PKG_VERSION).unwrap()).encoded(),
    )
    .await
    .map_err(|err| anyhow!("{}:{}: {err}", file!(), line!()))?;

    let frame = ws
        .receive()
        .await
        .map_err(|err| anyhow!("{}:{}: {err}", file!(), line!()))?;
    let payload = if let Frame::Binary {
        payload,
        continuation,
        fin,
    } = frame
    {
        if continuation || !fin {
            bail!(
                "{}:{}: continuation: {continuation}, fin: {fin}",
                file!(),
                line!()
            )
        }
        payload
    } else {
        bail!("{}:{}: {frame:?}", file!(), line!());
    };
    let message = ServerMessage::from_encoded(&payload)?;
    debug!("{}:{}: {message:?}", file!(), line!());
    if let ServerMessage::AgentVersion(res) = message {
        if let Err(err) = res.and_then(|need_refresh| {
            need_refresh
                .then(|| Err(format!("{PKG_VERSION}need_refresh")))
                .unwrap_or(Ok(()))
        })
        // .map_err(|err| anyhow!("{}:{}: {err}", file!(), line!()))
        {
            eprintln!("ERR: {err}");
            std::process::exit(1);
        }
    } else {
        bail!("{}:{}: {message:?}", file!(), line!());
    }
    Ok(())
}
pub fn connect_sync(res: ConnectResult, url: String, connect_retry_millis: u64) -> Result<()> {
    match res {
        Ok(ws) => {
            pasitos!(ws_read push_back Read {
                ws,
                url,
                connect_retry_millis,
            });
        }
        Err(err) => {
            error!("{err}");
            pasitos!(delay WsConnect { url, connect_retry_millis } for std::time::Duration::from_millis(connect_retry_millis));
        }
    }
    Ok(())
}

pub type WsReceiveResult = std::result::Result<Frame, WebSocketError>;
pub struct ReadResult(WsReceiveResult, WsRead);
pub async fn read(mut ws: WsRead) -> ReadResult {
    ReadResult(ws.receive().await, ws)
}
pub fn read_sync(
    ReadResult(res, ws): ReadResult,
    url: String,
    connect_retry_millis: u64,
) -> Result<()> {
    enum Next {
        FlushAndRead,
        FlushAndReconnect,
        Reconnect,
        CloseAndReconnect(String),
        Read,
    }
    let next = match res {
        Ok(frame) => match frame {
            Frame::Text { .. } => {
                // todo!()
                Next::Read
            }
            Frame::Binary { .. } => {
                // assert!(!continuation);
                // assert!(fin);
                // todo!();
                Next::Read
            }
            Frame::Ping { .. } => Next::FlushAndRead,
            Frame::Close { .. } => Next::FlushAndReconnect,
            Frame::Pong { .. } => Next::Read,
        },
        Err(WebSocketClosedError) => Next::Reconnect,
        Err(err) => {
            error!("{}:{}: {err:?}", file!(), line!());
            Next::CloseAndReconnect(format!("{err:?}"))
        }
    };
    match next {
        Next::FlushAndRead => {
            pasitos!(ws_write push_front Perform {
                action: PerformAction::Flush,
                and_then: PerformAndThen::Read{
                    ws,
                    url, connect_retry_millis
                }
            });
        }
        Next::FlushAndReconnect => {
            pasitos!(ws_write push_front Perform {
                action: PerformAction::Flush,
                and_then: PerformAndThen::Reconnect{
                    url, connect_retry_millis
                }
            });
        }
        Next::Reconnect => {
            pasitos!(ws_write push_front Perform {
                action: PerformAction::Nothing,
                and_then: PerformAndThen::Reconnect{
                    url, connect_retry_millis
                }
            });
        }
        Next::CloseAndReconnect(reason) => {
            pasitos!(ws_write push_front Perform {
                action: PerformAction::Close(reason),
                and_then: PerformAndThen::Reconnect{
                    url, connect_retry_millis
                }
            });
        }
        Next::Read => {
            pasitos!(ws_read push_back Read { ws, url, connect_retry_millis });
        }
    }
    Ok(())
}

pub enum PerformAction {
    Flush,
    Close(String),
    Nothing,
}
pub enum PerformAndThen {
    Reconnect {
        url: String,
        connect_retry_millis: u64,
    },
    Read {
        ws: WsRead,
        url: String,
        connect_retry_millis: u64,
    },
}
pub type PerformResult = Result<()>;
pub async fn perform(action: PerformAction, and_then: &PerformAndThen) -> PerformResult {
    let res = match action {
        PerformAction::Flush => {
            if let Some(ws) = &mut WS.write().await.as_mut() {
                ws.flush()
                    .await
                    .map_err(|err| anyhow!("{}:{}: {err}", file!(), line!()))
            } else {
                Ok(())
            }
        }
        PerformAction::Close(reason) => {
            if let Some(ws) = &mut WS.write().await.as_mut() {
                ws.close(Some((1011, reason)))
                    .await
                    .map_err(|err| anyhow!("{}:{}: {err}", file!(), line!()))
            } else {
                Ok(())
            }
        }
        PerformAction::Nothing => Ok(()),
    };
    if matches!(and_then, PerformAndThen::Reconnect { .. }) {
        *WS.write().await = None;
    }
    res
}
pub fn perform_sync(res: PerformResult, and_then: PerformAndThen) -> Result<()> {
    if let Err(err) = res {
        error!("{err}");
    }
    match and_then {
        PerformAndThen::Reconnect {
            url,
            connect_retry_millis,
        } => {
            pasitos!(ws push_back Connect { url, connect_retry_millis });
        }
        PerformAndThen::Read {
            ws,
            url,
            connect_retry_millis,
        } => {
            pasitos!(ws_read push_back Read { ws, url, connect_retry_millis });
        }
    }
    Ok(())
}

pub type SendResult = Result<()>;
pub async fn send(frame: Frame) -> SendResult {
    (*WS.write().await).as_mut().unwrap().send(frame).await?;
    Ok(())
}
pub fn send_sync(res: SendResult) -> Result<()> {
    if let Err(err) = res {
        error!("{err}");
    } else {
        println!(
            "{}: did send data to server",
            Utc::now()
                .with_timezone(&Moscow)
                .to_rfc3339_opts(SecondsFormat::Secs, false)
        );
    }
    Ok(())
}
