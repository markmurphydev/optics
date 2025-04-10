use std::marker::PhantomData;

trait ProfunctorFamily {
    type Profunctor<B, C>: Profunctor<B, C>;
}

trait Profunctor<B, C> {
    fn dimap<Pre, Post, A, D>(self, pre: Pre, post: Post) -> impl Profunctor<A, D>
    where
        Pre: Fn(A) -> B,
        Post: Fn(C) -> D;
}

// Example impl
trait FnProfunctor<B, C> {
    fn run(&self, b: B) -> C;
}

impl<F, B, C> Profunctor<B, C> for F
where
    F: FnProfunctor<B, C>,
{
    fn dimap<Pre, Post, A, D>(self, pre: Pre, post: Post) -> impl Profunctor<A, D>
    where
        Pre: Fn(A) -> B,
        Post: Fn(C) -> D,
    {
        Dimap {
            pre,
            f: self,
            post,
            a: PhantomData,
            b: PhantomData,
            c: PhantomData,
            d: PhantomData,
        }
    }
}

struct Dimap<Pre, F, Post, A, B, C, D>
where
    Pre: Fn(A) -> B,
    F: FnProfunctor<B, C>,
    Post: Fn(C) -> D,
{
    pre: Pre,
    f: F,
    post: Post,
    a: PhantomData<A>,
    b: PhantomData<B>,
    c: PhantomData<C>,
    d: PhantomData<D>,
}

impl<Pre, F, Post, A, B, C, D> FnProfunctor<A, D> for Dimap<Pre, F, Post, A, B, C, D>
where
    Pre: Fn(A) -> B,
    F: FnProfunctor<B, C>,
    Post: Fn(C) -> D,
{
    fn run(&self, a: A) -> D {
        (self.post)(self.f.run((self.pre)(a)))
    }
}

// trait Optic<K, R, U, S, T>: Sized {
//     
// }
// 
// impl <K, F, R, U, S, T> OpticP<K, F, R, U>

// Self: * -> * -> *
trait OpticP<F, R, U, S, T>: Sized
where
    F: ProfunctorFamily,
{
    fn transform(self, pro: F::Profunctor<S, T>) -> impl Profunctor<R, U>;

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

trait AdapterP<F, R, U, S, T>: OpticP<F, R, U, S, T>
where
    F: ProfunctorFamily,
{
}

struct Adapter<View, Review, R, U, S, T>
where
    View: Fn(R) -> S,
    Review: Fn(T) -> U,
{
    view: View,
    review: Review,
    r: PhantomData<R>,
    u: PhantomData<U>,
    s: PhantomData<S>,
    t: PhantomData<T>,
}

impl<F, View, Review, R, U, S, T> OpticP<F, R, U, S, T> for Adapter<View, Review, R, U, S, T>
where
    F: ProfunctorFamily,
    View: Fn(R) -> S,
    Review: Fn(T) -> U,
{
    fn transform(self, pro: F::Profunctor<S, T>) -> impl Profunctor<R, U> {
        pro.dimap(self.view, self.review)
    }
}

struct WithOptic<Outer, Inner, S, T> {
    // kind: PhantomData<Kind>,
    outer: Outer,
    inner: Inner,
    s: PhantomData<S>,
    t: PhantomData<T>,
}

// ==== Introduction ====

#[cfg(test)]
mod test {
    use super::*;
    use crate::profunctor::Profunctor;

    #[test]
    fn add123() {
        fn add1(x: u32) -> u32 {
            x + 1
        }

        fn add3(x: u32) -> u32 {
            x + 3
        }

        struct Add2;
        impl FnProfunctor<u32, u32> for Add2 {
            fn run(&self, b: u32) -> u32 {
                b + 2
            }
        }

        let add6 = Add2.dimap(add1, add3);
        // println!("add6: {}", add6.run(0))
    }
}
