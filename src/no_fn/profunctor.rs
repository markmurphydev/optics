pub trait ProfunctorFamily {
    type Pro<X, Y>: Profunctor<X, Y>;
}

pub(crate) trait Profunctor<X, Y> {
    type Pro<Pre: Profunctor<A, B>, Post: Profunctor<C, D>, A, B, C, D>: Profunctor<B, C>;

    fn dimap<Pre, Post, W, Z>(
        self,
        pre: Pre,
        post: Post,
    ) -> Self::Pro<Pre, Post, W, X, Y, Z>
    where
        Pre: Fn(W) -> X,
        Post: Fn(Y) -> Z;
}
