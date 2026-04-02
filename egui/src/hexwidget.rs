use hexgeo::geom::HexOrientation as _;

use crate::HexUi;

/// A [HexWidget] is responsible for rendering and interaction for a specific hex
pub trait HexWidget {
    fn hex_ui(self, hui: HexUi<'_>);
}

impl HexWidget for () {
    fn hex_ui(self, hui: HexUi<'_>) {
        let stroke = hui.style().visuals.widgets.inactive.fg_stroke;
        let vertices = hui.orientation().vertices();

        let mut prev = vertices[5];
        for v in vertices {
            hui.line([prev, v], stroke);
            prev = v;
        }
    }
}
