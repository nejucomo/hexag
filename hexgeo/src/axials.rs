use derive_more::{From, Into};
use emath::Pos2;

use crate::geom::HexOrientation;

#[derive(Copy, Clone, Debug, From, Into)]
pub struct Axials {
    pub q: isize,
    pub r: isize,
}

impl Axials {
    pub const CENTER: Axials = Axials::new(0, 0);

    #[inline]
    pub const fn new(q: isize, r: isize) -> Self {
        Axials { q, r }
    }

    #[inline]
    pub fn center_pt(self, orientation: impl HexOrientation) -> Pos2 {
        let Axials { q, r } = self;

        let qvec = q as f32 * orientation.q_basis();
        let rvec = r as f32 * orientation.r_basis();

        (qvec + rvec).to_pos2()
    }
}
