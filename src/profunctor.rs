use std::marker::PhantomData;

trait Profunctor<B, C> {
    type F;
    fn dimap<Pre, Post, A, D>(self, pre: Pre, post: Post) -> Dimap<Pre, Self::F, Post, A, B, C, D>
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
    type F = fn(&F, B) -> C;

    fn dimap<Pre, Post, A, D>(self, pre: Pre, post: Post) -> Dimap<Pre, Self::F, Post, A, B, C, D>
    where
        Pre: Fn(A) -> B,
        Post: Fn(C) -> D,
    {
        Dimap {
            pre,
            f: Self::run,
            post,
            a: PhantomData,
            b: PhantomData,
            c: PhantomData,
            d: PhantomData,
        }
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

// ==== Trying functors ====
// trait Functor<A> {
//     type With<F, B>: Functor<B>
//     where
//         F: Fn(A) -> B;
//
//     fn map<F, B>(self, f: F) -> Self::With<F, B>
//     where
//         F: Fn(A) -> B;
// }
//
// impl<A> Functor<A> for Option<A> {
//     type With<F, B>
//     where
//         F: Fn(A) -> B,
//     = Option<B>;
//
//     fn map<F, B>(self, f: F) -> Self::With<F, B>
//     where
//         F: Fn(A) -> B,
//     {
//         Option::map(self, f)
//     }
// }
//
// impl<A, E> Functor<A> for Result<A, E> {
//     type With<F, B>
//     where
//         F: Fn(A) -> B,
//     = Result<B, E>;
//
//     fn map<F, B>(self, f: F) -> Self::With<F, B>
//     where
//         F: Fn(A) -> B,
//     {
//         self.map(f)
//     }
// }

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

        let b = Add2.dimap(add1, add3);
    }
}
