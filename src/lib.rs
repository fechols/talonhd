//! TALON HD — flat-shaded 3D ships rendered offline into REU-DMA software
//! sprites (up to 512×512) over a vertical-scrolling 160×200 multicolor
//! bitmap on the C64 Ultimate.
//!
//! The crate is shared: without the `host` feature it is `no_std` and is
//! linked into the C64 program (`target-c64/`). The generic renderer in
//! [`render`] is the same code on both sides: on the C64 it drives the real
//! REU registers, on the host it drives [`model::Machine`], a byte-exact
//! model of C64 RAM plus the REU.

#![cfg_attr(not(feature = "host"), no_std)]

pub mod ace;
pub mod air2;
pub mod audio;
pub mod boss;
pub mod astrolabe;
pub mod campanile;
pub mod clear;
pub mod demo;
pub mod format;
pub mod game;
pub mod gnd2;
pub mod ground;
pub mod hud;
pub mod hw;
pub mod midboss;
pub mod paging;
pub mod layout;
pub mod render;
pub mod sim;
pub mod stage2;
pub mod uci;

#[cfg(feature = "host")]
pub mod bake;
#[cfg(feature = "host")]
pub mod encode;
#[cfg(feature = "host")]
pub mod font;
#[cfg(feature = "host")]
pub mod hand;
#[cfg(feature = "host")]
pub mod hand_s2;
#[cfg(feature = "host")]
pub mod mesh;
#[cfg(feature = "host")]
pub mod model;
#[cfg(feature = "host")]
pub mod models;
#[cfg(feature = "host")]
pub mod palette;
#[cfg(feature = "host")]
pub mod png;
#[cfg(feature = "host")]
pub mod raster3d;
#[cfg(feature = "host")]
pub mod reference;
#[cfg(feature = "host")]
pub mod s1;
#[cfg(feature = "host")]
pub mod s2;
