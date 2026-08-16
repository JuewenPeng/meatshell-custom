#![allow(dead_code, unused_imports)]

#[path = "impls/system.rs"]
pub(crate) mod system;
#[path = "struct/system.rs"]
mod system_types;

pub(crate) use system_types::{
    LocalGpuInfo, LocalHardwareInfo, LocalSnap, NetHist, TabStatus,
    TabStatuses,
};
pub(crate) use system_types::{SystemSampler, SystemSnapshot};
