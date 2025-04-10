use std::convert;
use std::marker::PhantomData;

// ==== Class Hierarchy ====
// For calculating least upper bound of two optics kinds

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

// ==== Families ====
// For emulating higher-kinded types

trait ProfunctorFamily {
    type Pro<'p, F, X, Y>: Profunctor<'p, X, Y>
    where
        F: Fn(X) -> Y + 'p,
        X: 'p,
        Y: 'p;
}

trait Profunctor<'p, X, Y> {
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

// ==== Reified Function ====
// Gives us something to return

struct FunctionProfunctorFamily;
impl ProfunctorFamily for FunctionProfunctorFamily {
    type Pro<'p, F, X, Y>
        = Function<F, X, Y>
    where
        F: Fn(X) -> Y + 'p,
        X: 'p,
        Y: 'p;
}

// struct Function<F, X, Y> {
//     f: F,
//     _xy: PhantomData<(X, Y)>,
// }
// 
// impl<F, X, Y> Function<F, X, Y> {
//     pub fn new(f: F) -> Self {
//         Function {
//             f,
//             _xy: Default::default(),
//         }
//     }
// }
// 
// impl<F, X, Y> Function<F, X, Y>
// where
//     F: Fn(X) -> Y,
// {
//     pub fn run(&self, a: X) -> Y {
//         (self.f)(a)
//     }
// }
// 
// impl<'p, F, X, Y> Profunctor<'p, X, Y> for Function<F, X, Y>
// where
//     F: Fn(X) -> Y + 'p,
//     X: 'p,
//     Y: 'p,
// {
//     type Family = FunctionProfunctorFamily;
// 
//     fn dimap<Pre, Post, W, Z>(
//         self,
//         pre: Pre,
//         post: Post,
//     ) -> <Self::Family as ProfunctorFamily>::Pro<'p, impl Fn(W) -> Z + 'p, W, Z>
//     where
//         Pre: Fn(W) -> X + 'p,
//         Post: Fn(Y) -> Z + 'p,
//         W: 'p,
//         Z: 'p,
//     {
//         Function::new(move |w| post(self.run(pre(w))))
//     }
// }

struct FnProfunctorFamily;
impl ProfunctorFamily for FnProfunctorFamily {
    type Pro<'p, F, X, Y>
        = F
    where
        F: Fn(X) -> Y + 'p,
        X: 'p,
        Y: 'p;
}

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

struct Dimap<Pre, F, Post, A, B, C, D> {
    pre: Pre,
    f: F,
    post: Post,
    a: PhantomData<A>,
    b: PhantomData<B>,
    c: PhantomData<C>,
    d: PhantomData<D>,
}
impl<Pre, F, Post, A, B, C, D> Dimap<Pre, F, Post, A, B, C, D> {
    pub fn new(pre: Pre, f: F, post: Post) -> Self {
        Dimap {
            pre,
            f,
            post,
            a: Default::default(),
            b: Default::default(),
            c: Default::default(),
            d: Default::default(),
        }
    }
}

// impl<Pre, F, Post, A, B, C, D> Profunctor<A, D> for Dimap<Pre, F, Post, A, B, C, D>
// where
//     Pre: Fn(A) -> B,
//     F: Profunctor<B, C>,
//     Post: Fn(C) -> D,
// {
//     fn dimap<OuterPre, OuterPost, X, Y>(
//         self,
//         pre: OuterPre,
//         post: OuterPost,
//     ) -> impl Profunctor<X, Y>
//     where
//         OuterPre: Fn(X) -> A,
//         OuterPost: Fn(D) -> Y,
//     {
//         Dimap::new(pre, self, post)
//     }
// }

trait Optic<C, R, U, S, T>: Sized
where
    C: OpticClass,
{
    fn transform(self, pro: impl Profunctor<S, T>) -> impl Profunctor<R, U>;

    // with : (p s t -> p r u) -> (p a b -> p s t) -> (p a b -> p r u)
    // Don't be deceived by the <S, T> in `WithOptic`. It's just phantomdata for the intermediates
    fn with<Inner, InnerClass, A, B>(
        self,
        inner: Inner,
    ) -> WithOptic<C::With<InnerClass>, Self, Inner, S, T>
    where
        InnerClass: OpticClass,
    {
        WithOptic::new(self, inner)
    }
}

// TODO -- move this to another module
trait TAdapter<C, R, U, S, T>: Optic<C, R, U, S, T>
where
    C: OpticClass,
{
    fn view(&self, structure: R) -> S {
        let concrete = Adapter::new(convert::identity, convert::identity);
        let concrete = self.transform(concrete);
        todo!()
    }
}

// impl<O, C, F, R, U, S, T> OpticP<C, F, R, U, S, T> for O
// where
//     F: ProfunctorFamily,
//     O: Optic<C, R, U, S, T>,
// {
//     fn transform(self, pro: F::Profunctor<S, T>) -> impl Profunctor<R, U> {
//
//     }
// }

// Self: * -> * -> *
// trait OpticP<C, F, R, U, S, T>: Sized
// where
//     C: OpticClass,
//     F: ProfunctorFamily,
// {
//     fn transform(self, pro: F::Profunctor<S, T>) -> impl Profunctor<R, U>;
//
// }

struct WithOptic<C, Outer, Inner, S, T> {
    kind: PhantomData<C>,
    outer: Outer,
    inner: Inner,
    s: PhantomData<S>,
    t: PhantomData<T>,
}

impl<C, Outer, Inner, S, T> WithOptic<C, Outer, Inner, S, T> {
    pub fn new(outer: Outer, inner: Inner) -> Self {
        Self {
            kind: Default::default(),
            outer,
            inner,
            s: Default::default(),
            t: Default::default(),
        }
    }
}

// ==== Introduction ====

struct Adapter<View, Review, R, U, S, T> {
    view: View,
    review: Review,
    r: PhantomData<R>,
    u: PhantomData<U>,
    s: PhantomData<S>,
    t: PhantomData<T>,
}

impl<View, Review, R, U, S, T> Adapter<View, Review, R, U, S, T> {
    pub fn new(view: View, review: Review) -> Self {
        Adapter {
            view,
            review,
            r: Default::default(),
            u: Default::default(),
            s: Default::default(),
            t: Default::default(),
        }
    }
}

impl<View, Review, R, U, S, T> Profunctor<R, U> for Adapter<View, Review, R, U, S, T>
where
    View: Fn(R) -> S,
    Review: Fn(T) -> U,
{
    fn dimap<Pre, Post, X, Y>(self, pre: Pre, post: Post) -> impl Profunctor<X, Y>
    where
        Pre: Fn(X) -> R,
        Post: Fn(U) -> Y,
    {
        Adapter::new(|x: X| (self.view)(pre(x)), |t: T| post((self.review)(t)))
    }
}

impl<View, Review, R, U, S, T> Optic<IsAdapter, R, U, S, T> for Adapter<View, Review, R, U, S, T>
where
    View: Fn(R) -> S,
    Review: Fn(T) -> U,
{
    fn transform(self, pro: impl Profunctor<S, T>) -> impl Profunctor<R, U> {
        pro.dimap(self.view, self.review)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::profunctor::Profunctor;

    #[test]
    fn add123() {
        fn add1(x: u32) -> u32 {
            x + 1
        }

        fn add2(x: u32) -> u32 {
            x + 2
        }

        fn add3(x: u32) -> u32 {
            x + 3
        }

        let add6 = Function::new(add2).dimap(add1, add3);

        let adapter = Adapter::new(add1, add3);
        // println!("add6: {}", add6.run(0))
    }
}
