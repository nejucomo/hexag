/// Precomputed coordinate bounds for quick conversion between axial coords and usize storage indices
///
/// # Performance Tradeoffs
///
/// This design is optimized for a relatively small radius and stores `O(radius^2)` [usize]s to reduce conversion cost. Note/TODO: there has been no profiling of this pre-computation approach versus a purely arithmetic conversion implementation for varying radii.
#[derive(Debug, Clone)]
pub struct CoordBounds {
    radius: isize,

    // row for each r in -R..=R
    row_starts: Vec<usize>,

    // row for each packed index
    index_rows: Vec<usize>,
}

impl CoordBounds {
    pub fn new(radius: usize) -> Self {
        let radius_i = radius as isize;
        let rows = 2 * radius + 1;
        let len = 1 + 3 * radius * (radius + 1);

        let mut row_starts = Vec::with_capacity(rows);
        let mut index_rows = Vec::with_capacity(len);

        let mut start = 0usize;
        for row in 0..rows {
            let r = row as isize - radius_i;
            let l = row_len(radius_i, r);

            row_starts.push(start);
            index_rows.extend(std::iter::repeat_n(row, l));

            start += l;
        }

        debug_assert_eq!(start, len);
        debug_assert_eq!(index_rows.len(), len);

        Self {
            radius: radius_i,
            row_starts,
            index_rows,
        }
    }

    #[inline]
    pub fn radius(&self) -> isize {
        self.radius
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.index_rows.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.index_rows.is_empty()
    }

    #[inline]
    pub fn contains(&self, q: isize, r: isize) -> bool {
        let s = -q - r;
        q.abs() <= self.radius && r.abs() <= self.radius && s.abs() <= self.radius
    }

    #[inline]
    pub fn axial_to_index(&self, q: isize, r: isize) -> Option<usize> {
        if !self.contains(q, r) {
            return None;
        }
        let row = self.row_of_r(r);
        Some(self.row_starts[row] + (q - q_min(self.radius, r)) as usize)
    }

    #[inline]
    pub fn index_to_axial(&self, index: usize) -> Option<(isize, isize)> {
        let &row = self.index_rows.get(index)?;
        let r = self.r_of_row(row);
        let q = q_min(self.radius, r) + (index - self.row_starts[row]) as isize;
        Some((q, r))
    }

    #[inline]
    pub fn row_bounds(&self, r: isize) -> Option<std::ops::Range<usize>> {
        if r < -self.radius || r > self.radius {
            return None;
        }
        let row = self.row_of_r(r);
        let start = self.row_starts[row];
        let end = start + row_len(self.radius, r);
        Some(start..end)
    }

    fn row_of_r(&self, r: isize) -> usize {
        (r + self.radius) as usize
    }

    fn r_of_row(&self, row: usize) -> isize {
        row as isize - self.radius
    }
}

fn q_min(radius: isize, r: isize) -> isize {
    if r <= 0 { -radius - r } else { -radius }
}

fn row_len(radius: isize, r: isize) -> usize {
    (2 * radius + 1 - r.abs()) as usize
}
