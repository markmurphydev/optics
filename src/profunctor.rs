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
    type Profunctor<B, C>: Profunctor<B, C>;
}

trait Profunctor<B, C> {
    fn dimap<Pre, Post, A, D>(self, pre: Pre, post: Post) -> impl Profunctor<A, D>
    where
        Pre: Fn(A) -> B,
        Post: Fn(C) -> D;
}

// ==== Reified Function ====
// To implement Profunctor.

// struct Function<F, A, B> {
//     f: F,
//     a: PhantomData<A>,
//     b: PhantomData<B>,
// }
//
// impl<F, A, B> Function<F, A, B> {
//     pub fn new(f: F) -> Self {
//         Function {
//             f,
//             a: Default::default(),
//             b: Default::default(),
//         }
//     }
// }
//
// impl<F, A, B> Function<F, A, B>
// where
//     F: Fn(A) -> B,
// {
//     pub fn run(&self, a: A) -> B {
//         (self.f)(a)
//     }
// }

impl<F, B, C> Profunctor<B, C> for F
where
    F: Fn(B) -> C,
{
    fn dimap<Pre, Post, A, D>(self, pre: Pre, post: Post) -> impl Profunctor<A, D>
    where
        Pre: Fn(A) -> B,
        Post: Fn(C) -> D,
    {
        Dimap::new(pre, self, post)
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

impl<Pre, F, Post, A, B, C, D> Profunctor<A, D> for Dimap<Pre, F, Post, A, B, C, D>
where
    Pre: Fn(A) -> B,
    F: Profunctor<B, C>,
    Post: Fn(C) -> D,
{
    fn dimap<OuterPre, OuterPost, X, Y>(
        self,
        pre: OuterPre,
        post: OuterPost,
    ) -> impl Profunctor<X, Y>
    where
        OuterPre: Fn(X) -> A,
        OuterPost: Fn(D) -> Y,
    {
        Dimap::new(pre, self, post)
    }
}

trait Optic<C, R, U, S, T>: Sized
where
    C: OpticClass,
{
    fn transform(self, pro: impl Profunctor<S, T>) -> impl Profunctor<R, U>;

    // with : (p s t -> p r u) -> (p a b -> p s t) -> (p a b) -> (p r u)
    // Don't be deceived by the <S, T> in `WithOptic`. It's just phantomdata for the intermediates
    fn with<Inner, A, B>(self, inner: Inner) -> WithOptic<Self, Inner, S, T> {
        WithOptic {
            outer: self,
            inner,
            s: Default::default(),
            t: Default::default(),
        }
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

struct WithOptic<Outer, Inner, S, T> {
    // kind: PhantomData<Kind>,
    outer: Outer,
    inner: Inner,
    s: PhantomData<S>,
    t: PhantomData<T>,
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
