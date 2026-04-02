//! [egui] and graphical support for [hexgeo]
//!
#![deny(unsafe_code)]

mod ext;
mod projector;
mod wireframe;

pub use self::ext::AxialsExt;
pub use self::wireframe::Wireframe;
