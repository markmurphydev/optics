mod concrete_impl;

use crate::concrete_impl::Lens;
use std::marker::PhantomData;

trait ProfSuper<'a, B, C> {
    type Target<T, U>;
}

/// Adapted from `higher` library
trait Profunctor<'a, B, C>: ProfSuper<'a, B, C> {
    fn dimap<A, D, Pre, Post>(self, pre: Pre, post: Post) -> Self::Target<A, D>
    where
        Pre: Fn(A) -> B + 'a,
        Post: Fn(C) -> D + 'a;
}

/// Something something GAT bounds
impl<'a, B, C, F> ProfSuper<'a, B, C> for F
where
    F: Fn(B) -> C,
{
    type Target<T, U> = Box<dyn Fn(T) -> U + 'a>;
}

impl<'a, B, C, F> Profunctor<'a, B, C> for F
where
    F: Fn(B) -> C + 'static,
{
    fn dimap<A, D, Pre, Post>(self, pre: Pre, post: Post) -> Self::Target<A, D>
    where
        Pre: Fn(A) -> B + 'a,
        Post: Fn(C) -> D + 'a,
    {
        Box::new(move |a| post(self(pre(a))))
    }
}

struct Optic<P1, P2, A, B, S, T>
where
    P1: for<'a> Profunctor<'a, A, B>,
    P2: for<'a> Profunctor<'a, S, T>,
{
    inner: Box<dyn Fn(P1) -> P2>,
    _b: PhantomData<A>,
    _c: PhantomData<B>,
    _s: PhantomData<S>,
    _t: PhantomData<T>,
}

// struct Lens<'a, A, B, S, T> {
//     view: Box<dyn Fn(S) -> A + 'a>,
//     update: Box<dyn Fn(B, S) -> T + 'a>,
// }

/// Reverse-composition
fn seq<F, G, A, B, C>(f: impl Fn(A) -> B, g: impl Fn(B) -> C) -> impl Fn(A) -> C {
    move |a| g(f(a))
}

fn cross<A, B, C, D>(f: impl Fn(A) -> B, g: impl Fn(C) -> D, (a, c): (A, C)) -> (B, D) {
    (f(a), g(c))
}

// impl ProfSuper for Lens a b
// impl<'a, A, B, S, T> ProfSuper<'a, S, T> for Lens<'a, A, B, S, T> {
//     // dimap : (Pre -> X) -> (Y -> Post) -> Lens X Y -> Lens Pre Post
//     type Target<U, V> = Lens<'a, A, B, U, V>;
// }

// impl<'a, A, B, S, T> Profunctor<'a, S, T> for Lens<'a, A, B, S, T> {
//     // HKT signature:
//     // dimap : (X -> S) -> (T -> Y) -> Profunctor S T -> Profunctor X Y
//     fn dimap<X, Y, Pre, Post>(self, pre: Pre, post: Post) -> Self::Target<X, Y>
//     where
//         Pre: Fn(X) -> S + 'a,
//         Post: Fn(T) -> Y + 'a,
//     {
//         // view : X -> A
//         let view = |x: X| -> A { (self.view)(pre(x)) };
//         // update : (B, X) -> Y
//         let update = |b: B, x: X| -> Y { post((self.update)(b, pre(x))) };
//         Lens {
//             view: Box::new(view),
//             update: Box::new(update),
//         }
//     }
// }

// fn convert<'a, S, T, A, B>(a: impl Profunctor<'a, A, B>) -> impl Profunctor<'a, S, T> {
//     todo!()
// }

// impl<'a, B, C> Prf<'a> for fn(B) -> C {
//     type B = B;
//     type C = C;
//     type Target<T, U> = Box<dyn Fn(T) -> U>;
//
//     fn dimap<A, D, Pre, Post>(self, pre: Pre, post: Post) -> Self::Target<A, D>
//     where
//         A: Sized,
//         D: Sized,
//         Pre: Fn(A) -> Self::B,
//         Post: Fn(Self::C) -> D,
//     {
//         // |a| {
//         //     post(self(pre))
//         // }
//         Box::new(|a| post(self(pre(a))))
//         // self(pre(a))
//     }
// }

// type Optic<A, B, S, T> = fn(impl Profunctor)
// type Optic<'a, A,B,S,T> = dyn Fn(dyn Profunctor<'a, A, B, Target=>) -> dyn Profunctor<'a, S, T>;

// impl<'a, B, C, F> Profunctor<'a, B, C> for F
// where
//     F: Fn(B) -> C + 'a,
// {
//     type Target<T, U> =
//
//     fn dimap<A, D, Pre, Post>(self, pre: Pre, post: Post) -> Self::Target<A, D>
//     where
//         Pre: Fn(A) -> B + 'a,
//         Post: Fn(C) -> D + 'a,
//     {
//         todo!()
//     }
// }

trait Functor {
    type Item;
    type Target<U>;
    fn fmap<U, F>(self, f: F) -> Self::Target<U>
    where
        F: FnMut(Self::Item) -> U;
}

fn main() {
    let ship_1 = Ship {
        captain: "Mark".to_string(),
        num_crew: 22,
    };

    let ship_2 = Ship {
        captain: "Kathryn".to_string(),
        num_crew: 44,
    };

    let ship_3 = Ship {
        captain: "Kristen".to_string(),
        num_crew: 88,
    };

    let armada = Armada { ship_1, ship_2 };

    // let ship_1_captain = armada::Ship1.with(ship::Captain);
    // dbg!(ship_1_captain.view(&armada));
    // dbg!(ship_1_captain.update("Kristen".to_string(), &armada));
}

// ==== Example ====
#[derive(Clone, Debug)]
pub struct Ship {
    pub captain: String,
    pub num_crew: u64,
}

pub mod ship {
    use crate::Ship;
    use crate::concrete_impl::*;

    // pub struct Captain;
    // impl Lens<Ship, Ship, String, String> for Captain {
    //     fn view<'a>(&'a self, ship: &'a Ship) -> &'a String {
    //         &ship.captain
    //     }
    //
    //     fn update(self, new_captain: String, old_ship: &Ship) -> Ship {
    //         Ship {
    //             captain: new_captain,
    //             ..old_ship.clone()
    //         }
    //     }
    // }
}

#[derive(Clone, Debug)]
pub struct Armada {
    pub ship_1: Ship,
    pub ship_2: Ship,
}

pub mod armada {
    use crate::concrete_impl::*;
    use crate::{Armada, Ship};

    pub struct Ship1;
    // impl Lens<Armada, Armada, Ship, Ship> for Ship1 {
    //     fn view<'a>(&'a self, armada: &'a Armada) -> &'a Ship {
    //         &armada.ship_1
    //     }
    //
    //     fn update(self, new_ship_1: Ship, old_armada: &Armada) -> Armada {
    //         Armada {
    //             ship_1: new_ship_1,
    //             ..old_armada.clone()
    //         }
    //     }
    // }
}
