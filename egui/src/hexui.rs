use std::sync::Arc;

use derive_new::new;
use egui::{Painter, Pos2, Stroke, Style, Ui};
use hexgeo::Axials;
use hexgeo::geom::DHO;

use crate::projector::Projector;

/// Handle to interact with `egui` scoped to a specific hex
#[derive(new)]
pub struct HexUi<'a> {
    ui: &'a mut Ui,
    p: &'a Projector,
    dho: DHO,
    ax: Axials,
}

impl<'a> HexUi<'a> {
    pub fn style(&self) -> &Arc<Style> {
        self.ui.style()
    }

    pub fn orientation(&self) -> DHO {
        self.dho
    }

    pub fn axials(&self) -> Axials {
        self.ax
    }

    pub fn line(&self, pts: [Pos2; 2], stroke: Stroke) {
        let screenpts = pts.map(|loc| self.local_to_screen(loc));

        self.painter().line_segment(screenpts, stroke);
    }

    fn painter(&self) -> &Painter {
        self.ui.painter()
    }

    fn local_to_screen(&self, pt: Pos2) -> Pos2 {
        // Translate to rim-logical pos:
        let rimpos = pt + self.ax.origin_to_center(self.dho);
        // Project into screen coordinates:
        self.p.project(rimpos)
    }
}
