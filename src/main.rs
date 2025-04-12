mod concrete_impl;
mod families;
mod hkt;
mod kind_in_generics;
mod optic;
mod profunctor;
mod testing;

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
