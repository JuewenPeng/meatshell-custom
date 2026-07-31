#[path = "struct/types.rs"]
mod types;

#[path = "impls/local.rs"]
pub(crate) mod local;
#[path = "impls/output_highlight.rs"]
mod output_highlight;
#[path = "impls/presentation.rs"]
mod presentation;
#[path = "impls/render.rs"]
mod render;
#[path = "impls/render_gate.rs"]
mod render_gate;
#[path = "impls/serial.rs"]
pub(crate) mod serial;
#[path = "impls/telnet.rs"]
pub(crate) mod telnet;
#[path = "impls/term_buffer.rs"]
mod term_buffer;
#[path = "impls/zmodem.rs"]
pub(crate) mod zmodem;

pub(crate) use presentation::{highlight_plain_output, render_term_span};
pub(crate) use render::{
    build_row, cell_prefix, char_after_cell_end, char_at_cell_start, detect_scroll, MAX_HISTORY,
    RAW_CAP,
};
pub(crate) use types::{
    BuiltScreen, CompiledOutputRule, CsiState, HistSpan, Line, OutputHighlightPreset, RenderGates,
    TabRenderGate, TermBuffer, TermBufferHandle, TermBuffers,
};
