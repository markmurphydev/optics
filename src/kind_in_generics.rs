//! Regime:
//! Every optic is first-and-foremost an `Optic<Kind, ...>`
//! where `Kind` is the _least-applicable-kind_
//! THEN:
//! we _only_ ever manually impl `Optic<Kind, ...>`
//! (even in `WithOptic<...>`)
//! And we have blanket impls `Optic -> (Lens, Prism, Traverse, ...)`
//! EDIT: other way
//! based on `Kind`.
//! TODO -- Where does behavior come from?

use std::marker::PhantomData;

struct IsOptic;
struct IsLens;
struct IsPrism;
struct IsIso;

trait OpticKind {
    type With<Rhs: OpticKind>;
    type WithOptic;
    type WithLens;
    type WithPrism;
    type WithIso;
}

impl OpticKind for IsOptic {
    type With<Rhs: OpticKind> = Rhs::WithOptic;
    type WithOptic = IsOptic;
    type WithLens = IsOptic;
    type WithPrism = IsOptic;
    type WithIso = IsOptic;
}

impl OpticKind for IsLens {
    type With<Rhs: OpticKind> = Rhs::WithLens;
    type WithOptic = IsOptic;
    type WithLens = IsLens;
    type WithPrism = IsOptic;
    type WithIso = IsLens;
}

impl OpticKind for IsPrism {
    type With<Rhs: OpticKind> = Rhs::WithPrism;
    type WithOptic = IsOptic;
    type WithLens = IsOptic;
    type WithPrism = IsPrism;
    type WithIso = IsPrism;
}

impl OpticKind for IsIso {
    type With<Rhs: OpticKind> = Rhs::WithIso;
    type WithOptic = IsOptic;
    type WithLens = IsLens;
    type WithPrism = IsPrism;
    type WithIso = IsIso;
}

trait Optic<Kind, R, U, S, T>: Sized
where
    Kind: OpticKind,
{
    fn with<Inner, InnerKind, A, B>(
        self,
        inner: Inner,
    ) -> WithOptic<Kind::With<InnerKind>, Self, Inner, S, T>
    where
        InnerKind: OpticKind,
        Inner: Optic<InnerKind, S, T, A, B>,
    {
        WithOptic {
            kind: PhantomData,
            outer: self,
            inner,
            s: PhantomData,
            t: PhantomData,
        }
    }
}

trait Lens<Kind, R, U, S, T>
where
    Kind: OpticKind,
{
}

trait Prism<Kind, R, U, S, T>
where
    Kind: OpticKind,
{
}

trait Iso<Kind, R, U, S, T>
where
    Kind: OpticKind,
{
}

impl<L, R, U, S, T> Optic<IsLens, R, U, S, T> for L where L: Lens<IsLens, R, U, S, T> {}

impl<P, R, U, S, T> Optic<IsPrism, R, U, S, T> for P where P: Prism<IsPrism, R, U, S, T> {}

impl<I, R, U, S, T> Optic<IsIso, R, U, S, T> for I where I: Iso<IsIso, R, U, S, T> {}
impl<I, R, U, S, T> Lens<IsIso, R, U, S, T> for I where I: Iso<IsIso, R, U, S, T> {}
impl<I, R, U, S, T> Prism<IsIso, R, U, S, T> for I where I: Iso<IsIso, R, U, S, T> {}

struct WithOptic<Kind, Outer, Inner, S, T> {
    kind: PhantomData<Kind>,
    outer: Outer,
    inner: Inner,
    s: PhantomData<S>,
    t: PhantomData<T>,
}

impl<Outer, Inner, R, U, S, T, A, B> Optic<IsOptic, R, U, A, B>
    for WithOptic<IsOptic, Outer, Inner, S, T>
{
}

impl<Outer, Inner, R, U, S, T, A, B> Lens<IsLens, R, U, A, B>
    for WithOptic<IsLens, Outer, Inner, S, T>
{
}

impl<Outer, Inner, R, U, S, T, A, B> Prism<IsPrism, R, U, A, B>
    for WithOptic<IsPrism, Outer, Inner, S, T>
{
}

impl<Outer, Inner, R, U, S, T, A, B> Iso<IsIso, R, U, A, B>
    for WithOptic<IsIso, Outer, Inner, S, T>
{
}
