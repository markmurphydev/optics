use std::marker::PhantomData;

pub trait Iso<R, U, S, T> {}

impl<I, R, U, S, T> Lens<R, U, S, T> for I
where
    I: Iso<R, U, S, T>,
{
    fn view<'a>(&'a self, structure: &'a R) -> &'a S {
        todo!()
    }

    fn update(self, new_focus: T, old_structure: &R) -> U {
        todo!()
    }
}

pub trait Lens<R, U, S, T>: Sized {
    fn view<'a>(&'a self, structure: &'a R) -> &'a S;
    fn update(self, new_focus: T, old_structure: &R) -> U;
    fn with<Inner, A, B>(self, inner: Inner) -> Composed<Self, Inner, S, T>
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

// impl<outer, inner, r, u, s, t, a, b> lens<r, u, a, b> for composed<outer, inner, s, t>
// where
//     outer: lens<r, u, s, t>,
//     inner: lens<s, t, a, b>,
// {
//     fn view<'a>(&'a self, structure: &'a r) -> &'a a {
//         self.inner.view(self.outer.view(structure))
//     }
//
//     fn update(self, new_focus: b, old_structure: &r) -> u {
//         let old_inner = self.outer.view(old_structure);
//         let new_inner = self.inner.update(new_focus, old_inner);
//         self.outer.update(new_inner, old_structure)
//     }
// }
