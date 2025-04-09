use std::marker::PhantomData;

pub trait Iso<R, U, S, T>
where
    R: Clone,
    U: Clone,
    S: Clone,
    T: Clone,
{
    fn view(&self, left: &R) -> S;
    fn review(&self, right: &T) -> U;
}

// impl<I, R: Clone, U: Clone, S: Clone, T: Clone> Lens<R, U, S, T> for I
// where
//     I: Iso<R, U, S, T>,
// {
//     fn view(&self, structure: &R) -> S {
//         self.view(structure)
//     }
//
//     fn update(self, new_focus: T, _old_structure: &R) -> U {
//         self.review(&new_focus)
//     }
// }

pub trait Lens<R: Clone, U: Clone, S: Clone, T: Clone>: Sized {
    fn view(&self, structure: &R) -> S;
    fn update(self, new_focus: T, old_structure: &R) -> U;
    fn with<Inner, A: Clone, B: Clone>(self, inner: Inner) -> Composed<Self, Inner, S, T>
    where
        Inner: Lens<S, T, A, B>,
    {
        Composed {
            outer: self,
            inner,
            s: PhantomData,
            t: PhantomData,
        }
    }
}

pub struct Composed<Outer, Inner, S, T> {
    outer: Outer,
    inner: Inner,
    s: PhantomData<S>,
    t: PhantomData<T>,
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
