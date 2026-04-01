//! [egui] and graphical support for [hexohexes]
//!
//! # References
//!
//! This crate, similar to many hexagon graphics/games trait relies heavily on the superb [Hexagonal Grids from Red Blob Games](https://www.redblobgames.com/grids/hexagons/).
#![deny(unsafe_code)]

mod ext;
mod orientation;
mod projector;
mod wireframe;

pub use self::ext::AxialsExt;
pub use self::orientation::HexOrientation;
pub use self::wireframe::Wireframe;
