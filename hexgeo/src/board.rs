use std::ops::{Deref, Index, IndexMut};

use crate::{Axials, RadialIndexMap};

#[derive(Clone, Debug)]
pub struct Board<T> {
    rim: RadialIndexMap,
    data: Vec<T>,
}

impl<T> Board<T> {
    pub fn new_defaults(radius: usize) -> Self
    where
        T: Default,
    {
        let bounds = RadialIndexMap::new(radius);

        let mut data = Vec::with_capacity(bounds.count());
        data.resize_with(bounds.count(), T::default);

        Board { rim: bounds, data }
    }

    pub fn rim(&self) -> &RadialIndexMap {
        &self.rim
    }

    pub fn get(&self, ax: Axials) -> Option<&T> {
        self.rim.axial_to_index(ax).map(|ix| &self.data[ix])
    }

    pub fn get_mut(&mut self, ax: Axials) -> Option<&mut T> {
        self.rim.axial_to_index(ax).map(|ix| &mut self.data[ix])
    }
}

impl<T> Deref for Board<T> {
    type Target = RadialIndexMap;

    fn deref(&self) -> &Self::Target {
        &self.rim
    }
}

impl<T> Index<Axials> for Board<T> {
    type Output = T;

    fn index(&self, ax: Axials) -> &Self::Output {
        &self.data[self.rim.require_axial_to_index::<Self>(ax)]
    }
}

impl<T> IndexMut<Axials> for Board<T> {
    fn index_mut(&mut self, ax: Axials) -> &mut Self::Output {
        &mut self.data[self.rim.require_axial_to_index::<Self>(ax)]
    }
}
