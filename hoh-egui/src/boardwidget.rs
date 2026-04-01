use derive_new::new;
use egui::{Color32, Response, Sense, Stroke, Ui, Widget};
use hexohexes::Board;

use crate::ext::AxialBoundsExt as _;
use crate::{AxialsExt as _, HexOrientation, Projector};

#[derive(Debug, new)]
pub struct BoardWidget<'a, T> {
    board: &'a Board<T>,
    hexor: HexOrientation,
}

impl<'a, T> Widget for BoardWidget<'a, T> {
    fn ui(self, ui: &mut Ui) -> Response {
        let resp = ui.allocate_rect(ui.max_rect(), Sense::hover());

        let projector = Projector::new(self.board.bounding_rect(self.hexor), ui.max_rect());
        let painter = ui.painter();

        for ax in self.board.iter_axials() {
            painter.circle_stroke(
                projector.project(ax.center_pos(self.hexor)),
                10.0,
                Stroke {
                    width: 1.0,
                    color: Color32::WHITE,
                },
            );
        }

        resp
    }
}
