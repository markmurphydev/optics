// ==== Class Hierarchy ====
// For calculating least upper bound of two optics kinds

use crate::with::{IsoWithIso, LensWithLens};
use std::marker::PhantomData;

struct IsOptic;
struct IsLens;
struct IsPrism;
struct IsAdapter;

trait OpticClass {
    type With<Rhs: OpticClass>;
    type WithOptic;
    type WithLens;
    type WithPrism;
    type WithAdapter;
}

impl OpticClass for IsOptic {
    type With<Rhs: OpticClass> = Rhs::WithOptic;
    type WithOptic = IsOptic;
    type WithLens = IsOptic;
    type WithPrism = IsOptic;
    type WithAdapter = IsOptic;
}

impl OpticClass for IsLens {
    type With<Rhs: OpticClass> = Rhs::WithLens;
    type WithOptic = IsOptic;
    type WithLens = IsLens;
    type WithPrism = IsOptic;
    type WithAdapter = IsLens;
}

impl OpticClass for IsPrism {
    type With<Rhs: OpticClass> = Rhs::WithPrism;
    type WithOptic = IsOptic;
    type WithLens = IsOptic;
    type WithPrism = IsPrism;
    type WithAdapter = IsPrism;
}

impl OpticClass for IsAdapter {
    type With<Rhs: OpticClass> = Rhs::WithAdapter;
    type WithOptic = IsOptic;
    type WithLens = IsLens;
    type WithPrism = IsPrism;
    type WithAdapter = IsAdapter;
}

// ==== Traits ====

// TODO -- If you want composition to work correctly, we need to descend in structure
//  Hense, <R, U, S, T, A, B>
//  REEEE
pub trait Lens<A, B, S, T> {
    fn view(&self, structure: &S) -> A;
    fn update(&self, new_focus: B, old_structure: &S) -> T;

    fn with_lens<L, X, Y>(self, lens: L) -> impl Lens<A, B, X, Y>
    where
        Self: Sized,
        L: Lens<S, T, X, Y>,
    {
        LensWithLens::new(self, lens)
    }
}

pub trait Iso<A, B, S, T> {
    fn view(&self, original: &S) -> A;
    fn review(&self, transformed: &B) -> T;

    fn with_iso<I, X, Y>(self, iso: I) -> impl Iso<A, B, X, Y>
    where
        Self: Sized,
        I: Iso<S, T, X, Y>,
    {
        IsoWithIso::new(self, iso)
    }
}

// ==== Implementations ====
// struct ConcreteLens<A, B, S, T, View, Update> {
//
// }

struct ConcreteIso<A, B, S, T, View, Review> {
    v: View,
    r: Review,
    _t: PhantomData<(A, B, S, T)>,
}

impl<A, B, S, T, View, Review> ConcreteIso<A, B, S, T, View, Review> {
    pub fn new(v: View, r: Review) -> Self {
        Self {
            v,
            r,
            _t: Default::default(),
        }
    }
}

impl<A, B, S, T, View, Review> Iso<S, T, A, B> for ConcreteIso<A, B, S, T, View, Review>
where
    View: Fn(&A) -> S,
    Review: Fn(&T) -> B,
{
    fn view(&self, left: &A) -> S {
        (self.v)(left)
    }

    fn review(&self, right: &T) -> B {
        (self.r)(right)
    }
}
