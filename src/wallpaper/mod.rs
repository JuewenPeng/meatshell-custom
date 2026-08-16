#![allow(dead_code, unused_imports)]

#[path = "struct/wallpaper.rs"]
mod wallpaper_types;
#[path = "impls/wallpaper.rs"]
mod wallpaper;

pub(crate) use wallpaper::*;
pub(crate) use wallpaper_types::*;
