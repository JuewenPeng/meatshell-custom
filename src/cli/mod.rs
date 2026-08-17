#[path = "impls/cli.rs"]
mod cli;

pub(crate) use cli::{is_cli_command, run};
