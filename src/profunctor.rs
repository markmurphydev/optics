// ==== Families ====
// For emulating higher-kinded types

pub(crate) trait ProfunctorFamily {
    type Pro<'p, F, X, Y>: Profunctor<'p, X, Y>
    where
        // You can't include `impl Fn(X) -> Y` in trait definition
        // But you _can_ include it as input to a GAT in a return type
        // So we just take it in from every `dimap`
        // _just in case_ the family is `FnProfunctorFamily`
        F: FnOnce(X) -> Y + 'p,
        X: 'p,
        Y: 'p;
}

pub(crate) trait Profunctor<'p, X, Y> {
    type Family: ProfunctorFamily;

    fn dimap<Pre, Post, W, Z, F>(
        self,
        pre: Pre,
        post: Post,
    ) -> <Self::Family as ProfunctorFamily>::Pro<'p, impl FnOnce(W) -> Z + 'p, W, Z>
    where
        Pre: Fn(W) -> X + 'p,
        Post: Fn(Y) -> Z + 'p,
        W: 'p,
        Z: 'p,
        F: FnOnce(W) -> Z + 'p;
}

// ==== impl Profunctor for Fn ====
pub struct FnProfunctorFamily;
impl ProfunctorFamily for FnProfunctorFamily {
    type Pro<'p, F, X, Y>
        = F
    where
        F: FnOnce(X) -> Y + 'p,
        X: 'p,
        Y: 'p;
}

impl<'p, F, X, Y> Profunctor<'p, X, Y> for F
where
    F: FnOnce(X) -> Y + 'p,
{
    type Family = FnProfunctorFamily;

    fn dimap<Pre, Post, W, Z, _F>(
        self,
        pre: Pre,
        post: Post,
    ) -> <Self::Family as ProfunctorFamily>::Pro<'p, impl FnOnce(W) -> Z + 'p, W, Z>
    where
        Pre: Fn(W) -> X + 'p,
        Post: Fn(Y) -> Z + 'p,
        W: 'p,
        Z: 'p,
    {
        move |w| post(self(pre(w)))
    }
}
