trait Min<Rhs> {
    type Result;
}

/// Double-dispatch!
trait HasMin {
    type Min<Rhs: HasMin>;
    type MinA;
    type MinB;
    type MinC;
}

trait A: B {}
trait B: C {}
trait C: HasMin {}

impl<T: A> B for T {}
impl<T: B> C for T {}

struct AImpl;
struct BImpl;
struct CImpl;

impl HasMin for CImpl {
    type Min<Rhs: HasMin> = Rhs::MinC;
    type MinA = AImpl;
    type MinB = BImpl;
    type MinC = CImpl;
}

// impl C for CImpl {}

fn to_min<X: C, Y: C>(a: X, b: Y) -> <X as HasMin>::Min<Y> {
    todo!()
}
