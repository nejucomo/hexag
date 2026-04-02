use egui::{Pos2, Rect};
use extension_traits::extension;
use hexgeo::{Axials, RadialIndexMap};

/// An extension trait to providing (normalized-scale) pixel values for [AxialBounds]
///
#[extension(pub trait AxialBoundsExt)]
impl RadialIndexMap {
    /// The bounding [Rect] such that all hexes fix entirely within this region.
    ///
    /// # Scaling
    ///
    /// This uses the same scaling as described in the [AxialsExt] trait docs.
    fn bounding_rect(&self, orientation: HexOrientation) -> Rect {
        let bl = orientation.bottom_left();
        let frad = self.radius() as f32;
        let width = orientation.q_basis().x * frad + bl.x;
        let height = orientation.r_basis().y * frad + bl.y;

        Rect::from_x_y_ranges(-width..=width, -height..=height)
    }
}
