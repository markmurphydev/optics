use crate::no_fn::profunctor::{Profunctor, ProfunctorFamily};

trait Fun<A, B>: Fn(A) -> B {
    fn then<C>(self, g: impl Fn(B) -> C) -> impl Fn(A) -> C  where Self: Sized{
        move |a| g(self(a))
    }
}
impl <F, A, B> Fun<A, B> for F where F: Fn(A) -> B {}

impl <F, X, Y> Profunctor<X, Y> for F where F: Fn(X) -> Y {
    type Pro<Pre: Profunctor<A, B>, Post: Profunctor<C, D>, A, B, C, D> = impl Fn(B) -> C;

    fn dimap<Pre, Post, W, Z>(self, pre: Pre, post: Post) -> Self::Pro<Pre, Post, W, X, Y, Z>
    where
        Pre: Fn(W) -> X,
        Post: Fn(Y) -> Z
    {
        move |w| post(self(pre(w)))
    }
}

#[cfg(test)]
mod test {
    use crate::no_fn::fun::Fun;

    #[test]
    fn do_with() {
        let add1 = |n| n + 1;
        let to_string = |n: i32| n.to_string();
        let add_1_then_to_string = add1.then(to_string);
        assert_eq!(add_1_then_to_string(2), "3");
    }
}