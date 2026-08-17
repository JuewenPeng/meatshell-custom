use std::time::Duration;

use anyhow::{anyhow, Result};
use serde_json::Value;

use crate::automation::Frontend;

/// Shared automation dispatch entry point. Individual CLI/MCP adapters can
/// build on the SFTP execution helpers without introducing another worker.
pub(crate) async fn call(name: &str, _arguments: &Value, _frontend: Frontend) -> Result<Value> {
    match name {
        "list_sessions" => list_sessions(),
        "list_remote_files" => list_remote_files(_arguments).await,
        "read_remote_text_file" => read_remote_text_file(_arguments).await,
        "upload_file" => upload_file(_arguments).await,
        "download_file" => download_file(_arguments).await,
        _ => Err(anyhow!("unknown automation tool: {name}")),
    }
}

fn list_sessions() -> Result<Value> {
    let store = crate::config::ConfigStore::load()?;
    let sessions = store
        .sessions()
        .iter()
        .map(|session| {
            serde_json::json!({
                "id": session.id,
                "name": session.name,
                "host": session.host,
                "port": session.port,
                "username": session.user,
                "group": session.group,
            })
        })
        .collect::<Vec<_>>();
    Ok(serde_json::json!({ "sessions": sessions }))
}

fn required_string(args: &Value, key: &str) -> Result<String> {
    args.get(key)
        .and_then(Value::as_str)
        .filter(|value| !value.trim().is_empty())
        .map(str::to_owned)
        .ok_or_else(|| anyhow!("missing required argument: {key}"))
}

fn session_context(args: &Value) -> Result<crate::config::Session> {
    let selector = args
        .get("session")
        .or_else(|| args.get("session_id"))
        .and_then(Value::as_str)
        .ok_or_else(|| anyhow!("missing required argument: session or session_id"))?;
    let store = crate::config::ConfigStore::load()?;
    store
        .sessions()
        .iter()
        .find(|session| session.id == selector || session.name == selector)
        .cloned()
        .ok_or_else(|| anyhow!("session not found: {selector}"))
}

async fn list_remote_files(args: &Value) -> Result<Value> {
    let session = session_context(args)?;
    let path = args
        .get("path")
        .and_then(Value::as_str)
        .unwrap_or("/")
        .to_string();
    super::sftp::list(session, None, path, Duration::from_secs(30)).await
}

async fn read_remote_text_file(args: &Value) -> Result<Value> {
    let session = session_context(args)?;
    let path = required_string(args, "path")?;
    super::sftp::read_text(session, None, path, Duration::from_secs(30)).await
}

async fn upload_file(args: &Value) -> Result<Value> {
    let session = session_context(args)?;
    let local = std::path::PathBuf::from(required_string(args, "local_path")?);
    if !local.is_file() {
        return Err(anyhow!("upload source is not a regular file: {}", local.display()));
    }
    let remote_dir = required_string(args, "remote_directory")?;
    super::sftp::transfer(
        session,
        None,
        crate::sftp::SftpCommand::Upload {
            local,
            remote_dir,
            cleanup_after: None,
            remote_name: None,
        },
        true,
        Duration::from_secs(300),
    )
    .await
}

async fn download_file(args: &Value) -> Result<Value> {
    let session = session_context(args)?;
    let remote = required_string(args, "remote_path")?;
    let local_dir = required_string(args, "local_directory")?;
    let conflict = match args.get("conflict").and_then(Value::as_str) {
        Some("keep_both") => crate::sftp::DownloadConflict::KeepBoth,
        _ => crate::sftp::DownloadConflict::Replace,
    };
    super::sftp::transfer(
        session,
        None,
        crate::sftp::SftpCommand::Download {
            remote,
            local_dir,
            conflict,
        },
        false,
        Duration::from_secs(300),
    )
    .await
}
