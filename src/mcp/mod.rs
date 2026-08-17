#[path = "impls/server.rs"]
mod server;

pub(crate) use server::run_stdio;
