use anyhow::{anyhow, Result};
use serde_json::json;

pub(crate) fn run(args: &[String]) -> Result<()> {
    let command = args.get(2).map(String::as_str).unwrap_or("help");
    if command == "help" {
        println!("meatshell cli <sessions|files|read|upload|download>");
        return Ok(());
    }
    let runtime = tokio::runtime::Builder::new_multi_thread().enable_all().build()?;
    let (name, arguments) = match command {
        "sessions" => ("list_sessions", json!({})),
        "files" => (
            "list_remote_files",
            json!({"session_id": args.get(3).ok_or_else(|| anyhow!("missing session id"))?, "path": args.get(4).map(String::as_str).unwrap_or("/")}),
        ),
        "read" => (
            "read_remote_text_file",
            json!({"session_id": args.get(3).ok_or_else(|| anyhow!("missing session id"))?, "path": args.get(4).ok_or_else(|| anyhow!("missing remote path"))?}),
        ),
        "upload" => (
            "upload_file",
            json!({"session_id": args.get(3).ok_or_else(|| anyhow!("missing session id"))?, "local_path": args.get(4).ok_or_else(|| anyhow!("missing local path"))?, "remote_directory": args.get(5).ok_or_else(|| anyhow!("missing remote directory"))?}),
        ),
        "download" => (
            "download_file",
            json!({"session_id": args.get(3).ok_or_else(|| anyhow!("missing session id"))?, "remote_path": args.get(4).ok_or_else(|| anyhow!("missing remote path"))?, "local_directory": args.get(5).ok_or_else(|| anyhow!("missing local directory"))?, "conflict": if args.iter().any(|a| a == "--keep-both") { "keep_both" } else { "replace" }}),
        ),
        _ => return Err(anyhow!("unknown CLI command: {command}")),
    };
    let value = runtime.block_on(crate::automation::call(name, &arguments, crate::automation::Frontend::Cli))?;
    println!("{}", serde_json::to_string_pretty(&value)?);
    Ok(())
}
