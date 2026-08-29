#[path = "impls/server.rs"]
mod server;

pub(crate) use server::{is_serve_command, run_stdio};
