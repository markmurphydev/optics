use std::marker::PhantomData;

trait OpticKind {
    type With<Rhs: OpticKind>;
    type WithIso;
    type WithLens;
    type WithOptic;
}

impl OpticKind for IsIso {
    type With<Rhs: OpticKind> = Rhs::WithIso;
    type WithIso = IsIso;
    type WithLens = IsLens;
    type WithOptic = IsOptic;
}

impl OpticKind for IsLens {
    type With<Rhs: OpticKind> = Rhs::WithLens;
    type WithIso = IsLens;
    type WithLens = IsLens;
    type WithOptic = IsOptic;
}

impl OpticKind for IsOptic {
    type With<Rhs: OpticKind> = Rhs::WithOptic;
    type WithIso = IsOptic;
    type WithLens = IsOptic;
    type WithOptic = IsOptic;
}

struct IsIso;
struct IsLens;
struct IsOptic;

// TODO -- Are the trait bounds needed?
// TODO -- Multiple inheritance (Lens / Prism) -> Traverse/Optic
pub trait Iso<R, U, S, T>
where
    R: Clone,
    U: Clone,
    S: Clone,
    T: Clone,
{
    type OpticKind: OpticKind;
    fn view(&self, left: &R) -> S;
    fn review(&self, right: &T) -> U;
}

impl<I, R, U, S, T> Lens<R, U, S, T> for I
where
    I: Iso<R, U, S, T>,
    R: Clone,
    U: Clone,
    S: Clone,
    T: Clone,
{
    type OpticKind = <I as Iso<R, U, S, T>>::OpticKind;

    fn view(&self, structure: &R) -> S {
        self.view(structure)
    }

    // We can recover the entire structure from `new_focus`,
    // so we can discard `old_structure`
    fn update(self, new_focus: T, _old_structure: &R) -> U {
        self.review(&new_focus)
    }
}

pub trait Lens<R, U, S, T>
where
    R: Clone,
    U: Clone,
    S: Clone,
    T: Clone,
{
    type OpticKind: OpticKind;

    fn view(&self, structure: &R) -> S;
    fn update(self, new_focus: T, old_structure: &R) -> U;
}

impl<L, R, U, S, T> Optic<R, U, S, T> for L
where
    L: Iso<R, U, S, T>,
    R: Clone,
    U: Clone,
    S: Clone,
    T: Clone,
{
    type OpticKind = <L as Lens<R, U, S, T>>::OpticKind;
}

pub trait Optic<R, U, S, T>: Sized {
    type OpticKind: OpticKind;

    fn with<Inner, A, B>(
        self,
        inner: Inner,
    ) -> Composed<Self, Inner, <Self::OpticKind as OpticKind>::With<Inner::OpticKind>>
    where
        Inner: Optic<S, T, A, B>,
        A: Clone,
        B: Clone,
    {
        Composed {
            outer: self,
            inner,
            composed_kind: PhantomData,
        }
    }
}

pub struct Composed<Outer, Inner, ComposedKind> {
    outer: Outer,
    inner: Inner,
    composed_kind: PhantomData<ComposedKind>,
}

impl<Outer, Inner, R, U, S, T, A, B> Iso<R, U, A, B> for Composed<Outer, Inner, S, T>
where
    Outer: Iso<R, U, S, T>,
    Inner: Iso<S, T, A, B>,
    R: Clone,
    U: Clone,
    S: Clone,
    T: Clone,
    A: Clone,
    B: Clone,
{
    type OpticKind = ();

    fn view(&self, left: &R) -> A {
        todo!()
    }

    fn review(&self, right: &B) -> U {
        todo!()
    }
}

// impl<Outer, Inner, R, U, S, T, A, B> Lens<R, U, A, B> for Composed<Outer, Inner, S, T>
// where
//     Outer: Lens<R, U, S, T>,
//     Inner: Lens<S, T, A, B>,
// {
//     fn view<'a>(&'a self, structure: &'a R) -> &'a A {
//         self.inner.view(self.outer.view(structure))
//     }
//
//     fn update(self, new_focus: B, old_structure: &R) -> U {
//         let old_inner = self.outer.view(old_structure);
//         let new_inner = self.inner.update(new_focus, old_inner);
//         self.outer.update(new_inner, old_structure)
//     }
// }
