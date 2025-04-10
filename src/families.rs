struct OpticsMin;
trait Min<Rhs> {
    type Result;
}

impl Min<OpticsIso> for OpticsIso {
    type Result = OpticsIso;
}

impl Min<OpticsLens> for OpticsIso {
    type Result = OpticsIso;
}

impl Min<OpticsIso> for OpticsLens {
    type Result = OpticsIso;
}

impl Min<OpticsLens> for OpticsLens {
    type Result = OpticsLens;
}

trait OpticsKind {}
struct OpticsIso;
impl OpticsKind for OpticsIso {}
struct OpticsLens;
impl OpticsKind for OpticsLens {}

trait Iso: Lens {
    type OpticsKind;
    fn from(&self, a: u32) -> u32;
}

trait Lens: Optic {
    type OpticsKind;
    fn view(&self, a: u32) -> u32;
}

trait Optic: Sized {
    type OpticsKind;
    fn with<Inner: Optic>(
        inner: Inner,
    ) -> Composed<Inner, Self, Inner::OpticsKind, Self::OpticsKind>;
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

    fn with<Inner: Optic>(
        inner: Inner,
    ) -> Composed<Inner, Self, Inner::OpticsKind, Self::OpticsKind> {
        todo!()
    }
}

struct Composed<Inner, Outer, InnerFamily, OuterFamily> {
    inner: Inner,
    inner_family: InnerFamily,
    outer: Outer,
    outer_family: OuterFamily,
}

impl<I, O> Iso for Composed<I, O, OpticsIso, OpticsIso>
where
    I: Iso,
    O: Iso,
{
    type OpticsKind = OpticsIso;

    fn from(&self, a: u32) -> u32 {
        a + 1
    }
}

impl<I, O> Iso for Composed<I, O, OpticsLens, OpticsIso>
where
    I: Iso,
    O: Iso,
{
    type OpticsKind = OpticsIso;

    fn from(&self, a: u32) -> u32 {
        a + 1
    }
}

impl<I, O> Iso for Composed<I, O, OpticsIso, OpticsLens>
where
    I: Iso,
    O: Iso,
{
    type OpticsKind = OpticsIso;

    fn from(&self, a: u32) -> u32 {
        a + 1
    }
}

impl<I, O> Lens for Composed<I, O, OpticsLens, OpticsLens>
where
    I: Lens,
    O: Lens,
{
    type OpticsKind = OpticsLens;

    fn view(&self, a: u32) -> u32 {
        todo!()
    }
}
