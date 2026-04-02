//! `hexgeo` - <u>hex</u>agonal <u>geo</u>metry and tiling utilities
//!
//! This crate, similar to many hexagon graphics/games trait relies heavily on the superb [Hexagonal Grids from Red Blob Games](https://www.redblobgames.com/grids/hexagons/).
//!
#![doc = include_str!("geom/overview.md")]
//!
//! See [geom] for more detail.
//!
#![doc = include_str!("geom/overview.svg")]
#![deny(unsafe_code)]

mod axials;
mod board;

pub mod geom;
pub mod radial;
pub use self::axials::Axials;
pub use self::board::Board;
pub use self::radial::RadialIndexMap;
