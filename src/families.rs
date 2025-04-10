use std::marker::PhantomData;

trait OpticsMin {
    type Min<Rhs: OpticsMin>;
    type MinIso;
    type MinLens;
    type MinOptic;
}

impl OpticsMin for IsoKind {
    type Min<Rhs: OpticsMin> = Rhs::MinIso;
    type MinIso = IsoKind;
    type MinLens = LensKind;
    type MinOptic = OpticKind;
}

impl OpticsMin for LensKind {
    type Min<Rhs: OpticsMin> = Rhs::MinLens;
    type MinIso = LensKind;
    type MinLens = LensKind;
    type MinOptic = OpticKind;
}

impl OpticsMin for OpticKind {
    type Min<Rhs: OpticsMin> = Rhs::MinOptic;
    type MinIso = OpticKind;
    type MinLens = OpticKind;
    type MinOptic = OpticKind;
}

struct IsoKind;
struct LensKind;
struct OpticKind;

trait Iso: Lens {
    type OpticsKind: OpticsMin;
    fn from(&self, a: u32) -> u32;
}

trait Lens: Optic {
    type OpticsKind: OpticsMin;
    fn view(&self, a: u32) -> u32;
}

trait Optic: Sized {
    type OpticsKind: OpticsMin;
    fn with<Inner: Optic>(
        self,
        inner: Inner,
    ) -> Composed<Inner, Self, <Self::OpticsKind as OpticsMin>::Min<Inner::OpticsKind>> {
        Composed {
            inner,
            outer: self,
            kind: PhantomData,
        }
    }
}

impl<I> Lens for I
where
    I: Iso,
{
    type OpticsKind = <I as Iso>::OpticsKind;

    fn view(&self, a: u32) -> u32 {
        self.from(a) + 2
    }
}

impl<L> Optic for L
where
    L: Lens,
{
    type OpticsKind = <L as Lens>::OpticsKind;

    // fn with<Inner: Optic>(
    //     inner: Inner,
    // ) -> Composed<Inner, Self, Inner::OpticsKind, Self::OpticsKind> {
    //     todo!()
    // }
}

struct Composed<Inner, Outer, OpticsKind> {
    inner: Inner,
    outer: Outer,
    kind: PhantomData<OpticsKind>,
}

impl<I, O> Iso for Composed<I, O, IsoKind>
where
    I: Iso,
    O: Iso,
{
    type OpticsKind = IsoKind;

    fn from(&self, a: u32) -> u32 {
        a + 1
    }
}

impl<I, O> Lens for Composed<I, O, LensKind>
where
    I: Lens,
    O: Lens,
{
    type OpticsKind = LensKind;

    fn view(&self, a: u32) -> u32 {
        todo!()
    }
}
