/// Entry point invoking the shared MeatShell automation capabilities.
///
/// MCP applies persisted permission gates because an external agent initiates
/// calls. CLI commands are explicit local user actions and therefore do not
/// depend on whether the MCP server itself is enabled.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum Frontend {
    Mcp,
    Cli,
}
