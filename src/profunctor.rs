// ==== Families ====
// For emulating higher-kinded types

pub(crate) trait ProfunctorFamily {
    type Pro<'p, F, X, Y>: Profunctor<'p, X, Y>
    where
        // You can't include `impl Fn(X) -> Y` in trait definition
        // But you _can_ include it as input to a GAT in a return type
        // So we just take it in from every `dimap`
        // _just in case_ the family is `FnProfunctorFamily`
        F: Fn(X) -> Y + 'p,
        X: 'p,
        Y: 'p;
}

pub(crate) trait Profunctor<'p, X, Y> {
    type Family: ProfunctorFamily;

    fn dimap<Pre, Post, W, Z>(
        self,
        pre: Pre,
        post: Post,
    ) -> <Self::Family as ProfunctorFamily>::Pro<'p, impl Fn(W) -> Z + 'p, W, Z>
    where
        Pre: Fn(W) -> X + 'p,
        Post: Fn(Y) -> Z + 'p,
        W: 'p,
        Z: 'p;
}

// pub(crate) trait SimpleProfunctorFamily {
//     type Pro<'p, F, X>: SimpleProfunctor<'p, X>;
// }
//
// pub(crate) trait SimpleProfunctor<'p, X> {
//     type Family: SimpleProfunctorFamily;
//
//     fn dimap<Pre, Post, Z>(
//         self,
//         pre: Pre,
//         post: Post,
//     ) -> <Self::Family as SimpleProfunctorFamily>::Pro<'p, impl Fn(Z) -> Z + 'p, Z>
//     where
//         Pre: Fn(W) -> X + 'p,
//         Post: Fn(Y) -> Z + 'p,
//         Z: 'p;
// }

// ==== impl Profunctor for Fn ====
struct FnProfunctorFamily;
impl ProfunctorFamily for FnProfunctorFamily {
    type Pro<'p, F, X, Y>
        = F
    where
        F: Fn(X) -> Y + 'p,
        X: 'p,
        Y: 'p;
}

// struct Fn1ProfunctorFamily;
// impl ProfunctorFamily for Fn1ProfunctorFamily {
//     type Pro<'p, F, X, Y>
//     where
//         F: Fn(X) -> Y + 'p,
//         X: 'p,
//         Y: 'p
//     = ();
// }

impl<'p, F, X, Y> Profunctor<'p, X, Y> for F
where
    F: Fn(X) -> Y + 'p,
{
    type Family = FnProfunctorFamily;

    fn dimap<Pre, Post, W, Z>(
        self,
        pre: Pre,
        post: Post,
    ) -> <Self::Family as ProfunctorFamily>::Pro<'p, impl Fn(W) -> Z + 'p, W, Z>
    where
        Pre: Fn(W) -> X + 'p,
        Post: Fn(Y) -> Z + 'p,
        W: 'p,
        Z: 'p,
    {
        move |w| post(self(pre(w)))
    }
}

// impl<'p, F, X> Profunctor<'p, X, X> for F
// where
//     F: Fn(X) -> X + 'p,
// {
//     type Family = FnProfunctorFamily;
//
//     fn dimap<Pre, Post, W, Z>(
//         self,
//         pre: Pre,
//         post: Post,
//     ) -> <Self::Family as ProfunctorFamily>::Pro<'p, impl Fn(W) -> Z + 'p, W, Z>
//     where
//         Pre: Fn(W) -> X + 'p,
//         Post: Fn(X) -> Z + 'p,
//         W: 'p,
//         Z: 'p,
//     {
//         |w| post(self(pre(w)))
//     }
// }

// trait Optic<'o, C, S, T, A, B>: Sized
// where
//     C: OpticClass,
// {
//     fn transform<P>(
//         &self,
//         profunctor: P,
//     ) -> <P::Family as ProfunctorFamily>::Pro<'o, impl Fn(S) -> T, S, T>
//     where
//         P: Profunctor<'o, A, B>;
//
//     // with : (p s t -> p r u) -> (p a b -> p s t) -> (p a b -> p r u)
//     // Don't be deceived by the <S, T> in `WithOptic`. It's just phantomdata for the intermediates
//     fn with<Inner, InnerClass, X, Y>(
//         self,
//         inner: Inner,
//     ) -> WithOptic<C::With<InnerClass>, Self, Inner, A, B>
//     where
//         InnerClass: OpticClass,
//     {
//         WithOptic::new(self, inner)
//     }
// }
//
// // // TODO -- move this to another module
// // trait TAdapter<C, R, U, S, T>: Optic<C, R, U, S, T>
// // where
// //     C: OpticClass,
// // {
// //     fn view(&self, structure: R) -> S {
// //         let concrete = Adapter::new(convert::identity, convert::identity);
// //         let concrete = self.transform(concrete);
// //         todo!()
// //     }
// // }
//
// // impl<O, C, F, R, U, S, T> OpticP<C, F, R, U, S, T> for O
// // where
// //     F: ProfunctorFamily,
// //     O: Optic<C, R, U, S, T>,
// // {
// //     fn transform(self, pro: F::Profunctor<S, T>) -> impl Profunctor<R, U> {
// //
// //     }
// // }
//
// // Self: * -> * -> *
// // trait OpticP<C, F, R, U, S, T>: Sized
// // where
// //     C: OpticClass,
// //     F: ProfunctorFamily,
// // {
// //     fn transform(self, pro: F::Profunctor<S, T>) -> impl Profunctor<R, U>;
// //
// // }
//
// struct WithOptic<C, Outer, Inner, S, T> {
//     outer: Outer,
//     inner: Inner,
//     _t: PhantomData<(C, S, T)>,
// }
//
// impl<C, Outer, Inner, S, T> WithOptic<C, Outer, Inner, S, T> {
//     pub fn new(outer: Outer, inner: Inner) -> Self {
//         Self {
//             outer,
//             inner,
//             _t: Default::default(),
//         }
//     }
// }
//
// // ==== Introduction ====
//
// struct Adapter<View, Review, S, T, A, B> {
//     view: View,
//     review: Review,
//     _t: PhantomData<(S, T, A, B)>,
// }
//
// impl<View, Review, R, U, S, T> Adapter<View, Review, R, U, S, T>
// where
//     View: Fn(R) -> S,
//     Review: Fn(T) -> U,
// {
//     pub fn new(view: View, review: Review) -> Self {
//         Adapter {
//             view,
//             review,
//             _t: Default::default(),
//         }
//     }
// }
//
// impl<'p, View, Review, R, U, S, T> Profunctor<R, U> for Adapter<View, Review, R, U, S, T>
// where
//     View: Fn(R) -> S,
//     Review: Fn(T) -> U,
// {
// }
//
// impl<View, Review, R, U, S, T> Optic<IsAdapter, R, U, S, T> for Adapter<View, Review, R, U, S, T>
// where
//     View: Fn(R) -> S,
//     Review: Fn(T) -> U,
// {
//     fn transform(self, pro: impl Profunctor<S, T>) -> impl Profunctor<R, U> {
//         pro.dimap(self.view, self.review)
//     }
// }
//
// #[cfg(test)]
// mod test {
//     use super::*;
//     use crate::profunctor::Profunctor;
//
//     #[test]
//     fn add123() {
//         fn add1(x: u32) -> u32 {
//             x + 1
//         }
//
//         fn add2(x: u32) -> u32 {
//             x + 2
//         }
//
//         fn add3(x: u32) -> u32 {
//             x + 3
//         }
//
//         let add6 = Function::new(add2).dimap(add1, add3);
//
//         let adapter = Adapter::new(add1, add3);
//         // println!("add6: {}", add6.run(0))
//     }
// }
