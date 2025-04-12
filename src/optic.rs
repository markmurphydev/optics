use crate::profunctor::{FnProfunctorFamily, Profunctor, ProfunctorFamily};
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

// ==== Optic ====
trait Optic<'o, A, B, S, T>
where
    A: 'o,
    B: 'o,
    S: 'o,
    T: 'o,
{
    fn transform<P>(&self, inner: P) -> <P::Family as ProfunctorFamily>::Pro<'o, fn(S) -> T, S, T>
    where
        P: Profunctor<'o, A, B>;

    // TODO -- do concrete impls first
    // fn then<Then, H, X, Y>(self, then: Then) -> impl Optic<'o, A, B, X, Y, PF>
    // where
    //     Self: Sized,
    //     Then: Optic<'o, S, T, X, Y, PF>,
    //     X: 'o,
    //     Y: 'o,
    // {
    //     move |a_to_b| then(self(a_to_b))
    // }
}

trait SimpleOptic<'o, A, S>
where
    A: 'o,
    S: 'o,
{
    fn transform<P>(&self, inner: P) -> <P::Family as ProfunctorFamily>::Pro<'o, fn(S) -> S, S, S>
    where
        P: Profunctor<'o, A, A>;
}

impl<'o, T, A, S> Optic<'o, A, A, S, S> for T
where
    T: SimpleOptic<'o, A, S>,
    A: 'o,
    S: 'o,
{
    fn transform<P>(&self, inner: P) -> <P::Family as ProfunctorFamily>::Pro<'o, fn(S) -> S, S, S>
    where
        P: Profunctor<'o, A, A>,
    {
        self.transform(inner)
    }
}

// struct FnBuilder;
//
//
// struct FnOptic<F, PF> {
//     f: F,
//     _t: PF,
// }
//
// impl<'o, A, B, S, T, F, PF> Optic<'o, A, B, S, T> for FnOptic<F, PF>
// where
//     // Needs to be generic
//     F: Fn(PF::Pro<'o, fn(A) -> B, A, B>) -> PF::Pro<'o, fn(S) -> T, S, T>,
//     PF: ProfunctorFamily,
//     A: 'o,
//     B: 'o,
//     S: 'o,
//     T: 'o,
// {
//     fn transform(inner: PF::Pro<'o, fn(A) -> B, A, B>) -> PF::Pro<'o, fn(S) -> T, S, T>
//     where
//         PF: ProfunctorFamily,
//     {
//         todo!()
//     }
// }

// ==== Adapter ====
struct KeepRightFamily;
impl ProfunctorFamily for KeepRightFamily {
    type Pro<'p, F, X, Y>
        = KeepRight<X, Y>
    where
        F: FnOnce(X) -> Y + 'p,
        X: 'p,
        Y: 'p;
}

struct KeepRight<X, Y> {
    y: Y,
    _t: PhantomData<X>,
}

impl<X, Y> KeepRight<X, Y> {
    fn new(y: Y) -> Self {
        KeepRight {
            y,
            _t: Default::default(),
        }
    }
}

impl<'p, X, Y> Profunctor<'p, X, Y> for KeepRight<X, Y> {
    type Family = KeepRightFamily;

    fn dimap<Pre, Post, W, Z>(
        self,
        pre: Pre,
        post: Post,
    ) -> <KeepRightFamily as ProfunctorFamily>::Pro<'p, fn(W) -> Z, W, Z>
    where
        Pre: Fn(W) -> X + 'p,
        Post: Fn(Y) -> Z + 'p,
        W: 'p,
        Z: 'p,
    {
        let a: KeepRight<W, Z> = KeepRight::new(post(self.y));
        a
    }
}

trait Adapter<'o, A, S>: Optic<'o, A, A, S, S>
where
    A: 'o,
    S: 'o,
{
    fn view(&self, structure: S) -> A {
        fn id<A>(a: A) -> A {
            a
        }
        fn constant<X, Y>(x: X) -> impl FnOnce(Y) -> X {
            move |_: Y| x
        }

        fn prove_id<A: 'static>() -> impl Profunctor<'static, A, A> {
            id
        }
        self.transform::<fn(S) -> S>(constant);

        fn prove_profunctor<S: 'static, A>() -> impl Profunctor<'static, S, S> {
            let s: S = todo!();
            constant(s)
        }

        let x = constant::<S, A>(structure);
        // let q: Box<dyn Profunctor<'static, S, A, Family = FnProfunctorFamily>> = Box::new(x);
        // Need to transform `fn(A) -> B=A` to `fn(S) -> T=A`

        // Self: p a b -> p s t
        // constant: p _ s
        // self.transform prepends (view: s -> a)

        let s_to_a = self.transform(x);

        todo!()
    }
}
