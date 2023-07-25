use super::*;

#[derive(Debug)]
pub enum Need {
    Remove(PathBuf),
    Reread(PathBuf),
}
use ipado3_common::AgentFile;

pub type Rx = tokio::sync::mpsc::Receiver<Need>;
pub struct ReceiveEventResult(ReceiveEventHelperResult, Rx);
pub async fn receive_event(mut rx: Rx) -> ReceiveEventResult {
    ReceiveEventResult(receive_event_helper(&mut rx).await, rx)
}

use tokio::fs::File;
use tokio::io::AsyncReadExt; // for read_to_end()
pub type ReceiveEventHelperResult = Result<Option<AgentFile>>;
async fn receive_event_helper(rx: &mut Rx) -> ReceiveEventHelperResult {
    match rx.recv().await {
        Some(Need::Remove(file_path)) => {
            let file_name = file_path.file_name().unwrap().to_string_lossy().to_string();
            FILES.write().await.remove(&file_name);
            Ok(Some(AgentFile::Removed(file_name)))
        }
        Some(Need::Reread(file_path)) => {
            info!("need to reread {file_path:?}");
            let mut file = File::open(&file_path).await?;

            let mut file_content = vec![];
            file.read_to_end(&mut file_content).await?;
            let checksum = adler::adler32_slice(&file_content);

            let file_name = file_path.file_name().unwrap().to_string_lossy().to_string();

            let checksum_prev = FILES.read().await.get(&file_name).cloned();
            let need_send = if let Some(checksum_prev) = checksum_prev {
                checksum_prev != checksum
            } else {
                true
            };
            Ok(if !need_send {
                None
            } else {
                FILES.write().await.insert(file_name.clone(), checksum);
                Some(AgentFile::Changed(AgentFileChanged {
                    file_name,
                    file_content,
                }))
            })
        }
        None => {
            bail!("watcher::receive_event_helper: failed to recv");
        }
    }
}

pub fn receive_event_sync(ReceiveEventResult(res, rx): ReceiveEventResult) -> Result<()> {
    let message = match res {
        Ok(Some(AgentFile::Changed(data))) => {
            #[allow(clippy::redundant_clone)]
            let file_name = data.file_name.clone();
            use flate2::write::GzEncoder;
            use flate2::Compression;
            use std::io::prelude::*;
            let mut e = GzEncoder::new(Vec::new(), Compression::default());
            let encoded = bincode::serialize(&data)
                .map_err(|err| format!("{err}"))
                .and_then(|encoded| {
                    e.write_all(&encoded)
                        .map_err(|err| format!("{err}"))
                        .and_then(|_| e.finish().map_err(|err| format!("{err}")))
                });
            println!(
                "{}: did prepare data of file {file_name:?} to send to server",
                Utc::now()
                    .with_timezone(&Moscow)
                    .to_rfc3339_opts(SecondsFormat::Secs, false)
            );
            Some(ClientMessage::AgentFileChanged(encoded))
        }
        Ok(Some(AgentFile::Removed(file_name))) => Some(ClientMessage::AgentFileRemoved(file_name)),
        Ok(None) => None,
        Err(err) => {
            warn!("{}:{} {err:?}", file!(), line!());
            None
        }
    };
    if let Some(message) = message {
        let frame = websockets::Frame::Binary {
            payload: message.encoded(),
            continuation: false,
            fin: true,
        };
        pasitos!(ws_write push_back Send { frame });
    }
    pasitos!(watcher push_back ReceiveEvent { rx });
    Ok(())
}

pub async fn run(opt: Opt) -> Result<Rx> {
    let path_to_be_watched = opt.path_to_be_watched.unwrap_or_else(|| PathBuf::from("."));
    let path_to_be_watched = path_to_be_watched
        .canonicalize()
        .map_err(|err| anyhow!("{:?}: {err}", path_to_be_watched))?;
    let filter_by_ext = opt.filter_by_ext;

    let mut dir = tokio::fs::read_dir(&path_to_be_watched)
        .await
        .map_err(|err| anyhow!("read_dir{:?}: {}", path_to_be_watched, err))?;
    let mut file_contents = std::collections::HashMap::new();
    info!("will prepare AgentInit of {path_to_be_watched:?}");
    while let Some(entry) = dir.next_entry().await? {
        if entry.metadata().await?.is_file() {
            let file_name = entry.file_name().to_string_lossy().to_string();
            if let Some(ext) = PathBuf::from(&file_name).extension() {
                if ext.to_string_lossy() != filter_by_ext.as_str() {
                    continue;
                }
            }
            let file_path = path_to_be_watched.join(&file_name);
            let mut file = File::open(&file_path).await?;
            let mut file_content = vec![];
            file.read_to_end(&mut file_content).await?;
            let checksum = adler::adler32_slice(&file_content);
            FILES.write().await.insert(file_name.clone(), checksum);
            file_contents.insert(file_name, file_content);
        }
    }
    info!("did prepare AgentInit of {} files", file_contents.len());
    *AGENT_INIT_TO_SEND.write().await = Some(AgentInit { file_contents });

    send_agent_init().await?;

    use notify::RecursiveMode::*;
    let debounce_millis = opt.debounce_millis;
    eprintln!(
        "will watch {path_to_be_watched:?} with {debounce_millis} millis debounce and {}",
        if filter_by_ext.is_empty() {
            "no filter by ext".to_owned()
        } else {
            format!("filter by ext {filter_by_ext:?}")
        }
    );

    let (tx_async, rx) = tokio::sync::mpsc::channel(WATCHER_CHANNEL_SIZE);

    std::thread::spawn(move || {
        use notify::Watcher;
        let (tx, rx) = std::sync::mpsc::channel();
        let mut watcher =
            notify::watcher(tx, std::time::Duration::from_millis(debounce_millis)).unwrap();
        let recursive_mode = NonRecursive;
        if let Err(err) = watcher.watch(&path_to_be_watched, recursive_mode) {
            panic!("{path_to_be_watched:?}: {err}");
        }
        loop {
            use notify::DebouncedEvent::*;
            match rx.recv() {
                Err(err) => error!("watch error: {err:?}"),
                Ok(event) => match event {
                    Remove(old_file_path) => {
                        file_removed(&filter_by_ext, old_file_path, &tx_async);
                    }
                    Rename(old_file_path, file_path) => {
                        file_removed(&filter_by_ext, old_file_path, &tx_async);
                        file_changed(&filter_by_ext, file_path, &tx_async);
                    }
                    NoticeWrite(file_path)
                    | Create(file_path)
                    | Write(file_path)
                    | Chmod(file_path) => {
                        file_changed(&filter_by_ext, file_path, &tx_async);
                    }
                    _ => debug!("{}:{} {event:?}", file!(), line!()),
                },
            }
        }
    });

    Ok(rx)
}

fn file_removed(
    filter_by_ext: &str,
    file_path: PathBuf,
    tx_async: &tokio::sync::mpsc::Sender<Need>,
) {
    if need_send(filter_by_ext, &file_path) {
        println!(
            "{}: removed {file_path:?}",
            Utc::now()
                .with_timezone(&Moscow)
                .to_rfc3339_opts(SecondsFormat::Secs, true)
        );
        if let Err(err) = tx_async.try_send(Need::Remove(file_path)) {
            eprintln!(
                concat!("ERR: ", file!(), ":", line!(), ": try_send error: {:?}"),
                err
            );
        }
    } else {
        debug!("{}:{} ignore {file_path:?} changes", file!(), line!())
    }
}

fn file_changed(
    filter_by_ext: &str,
    file_path: PathBuf,
    tx_async: &tokio::sync::mpsc::Sender<Need>,
) {
    if need_send(filter_by_ext, &file_path) {
        println!(
            "{}: changed {file_path:?}",
            Utc::now()
                .with_timezone(&Moscow)
                .to_rfc3339_opts(SecondsFormat::Secs, true)
        );
        if let Err(err) = tx_async.try_send(Need::Reread(file_path)) {
            eprintln!(
                concat!("ERR: ", file!(), ":", line!(), ": try_send error: {:?}"),
                err
            );
        }
    } else {
        debug!("{}:{} ignore {file_path:?} changes", file!(), line!())
    }
}

use std::path::{Path, PathBuf};
fn need_send(filter_by_ext: &str, file_path: &Path) -> bool {
    if file_path
        .file_name()
        .map(|file_name| file_name.to_string_lossy().starts_with('~'))
        .unwrap_or(true)
    {
        false
    } else if filter_by_ext.is_empty() {
        true
    } else if let Some(ext) = file_path.extension() {
        ext.to_string_lossy() == filter_by_ext
    } else {
        false
    }
}
