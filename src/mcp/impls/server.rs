use anyhow::Result;
use serde_json::{json, Value};
use std::io::{self, BufRead, Write};

pub(crate) fn is_serve_command(args: &[String]) -> bool {
    args.get(1).is_some_and(|value| value == "serve")
}

pub(crate) fn run_stdio() -> Result<()> {
    let runtime = tokio::runtime::Builder::new_multi_thread().enable_all().build()?;
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    for line in stdin.lock().lines() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }
        let request: Value = match serde_json::from_str(&line) {
            Ok(value) => value,
            Err(error) => {
                writeln!(stdout, "{}", json!({"error": error.to_string()}))?;
                stdout.flush()?;
                continue;
            }
        };
        let name = request
            .get("name")
            .or_else(|| request.get("method"))
            .and_then(Value::as_str)
            .unwrap_or_default();
        let arguments = request.get("arguments").unwrap_or(&Value::Null);
        let result = runtime.block_on(crate::automation::call(
            name,
            arguments,
            crate::automation::Frontend::Mcp,
        ));
        let response = match result {
            Ok(value) => json!({"id": request.get("id"), "result": value}),
            Err(error) => json!({"id": request.get("id"), "error": error.to_string()}),
        };
        writeln!(stdout, "{}", response)?;
        stdout.flush()?;
    }
    Ok(())
}
