use crate::concrete::{Iso, Lens};
use std::marker::PhantomData;

// ==== Iso with ____ ====

pub struct IsoWithIso<Left, Right, S, T> {
    left: Left,
    right: Right,
    _t: PhantomData<(S, T)>,
}

impl<Left, Right, S, T> IsoWithIso<Left, Right, S, T> {
    pub fn new(left: Left, right: Right) -> Self {
        Self {
            left,
            right,
            _t: Default::default(),
        }
    }
}

impl<A, B, S, T, X, Y, Left, Right> Iso<A, B, X, Y> for IsoWithIso<Left, Right, S, T>
where
    Left: Iso<A, B, S, T>,
    Right: Iso<S, T, X, Y>,
{
    fn view(&self, original: &X) -> A {
        self.left.view(&self.right.view(original))
    }

    fn review(&self, transformed: &B) -> Y {
        self.right.review(&self.left.review(transformed))
    }
}

// ==== Lens with ____ ====

pub struct LensWithLens<Left, Right, S, T> {
    left: Left,
    right: Right,
    _t: PhantomData<(S, T)>,
}
impl<Left, Right, S, T> LensWithLens<Left, Right, S, T> {
    pub fn new(left: Left, right: Right) -> Self {
        Self {
            left,
            right,
            _t: Default::default(),
        }
    }
}

impl<A, B, S, T, X, Y, Left, Right> Lens<A, B, X, Y> for LensWithLens<Left, Right, S, T>
where
    Left: Lens<A, B, S, T>,
    Right: Lens<S, T, X, Y>,
{
    fn view(&self, structure: &X) -> A {
        self.left.view(&self.right.view(structure))
    }

    fn update(&self, new_focus: B, old_structure: &X) -> Y {
        let new_middle = self.left.update(new_focus, &self.right.view(old_structure));
        self.right.update(new_middle, old_structure)
    }
}
