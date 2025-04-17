#![feature(impl_trait_in_assoc_type)]

use crate::concrete::Lens;

mod concrete;
mod kind_in_generics;
mod optic;
mod profunctor;
mod testing;
mod util;
mod with;
mod no_fn;

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
    //
    // let ship_1_captain = armada::Ship1.with_lens(ship::Captain);
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
    use crate::concrete::*;

    pub struct Captain;
    impl Lens<String, String, Ship, Ship> for Captain {
        fn view(&self, ship: &Ship) -> String {
            ship.captain.clone()
        }

        fn update(&self, new_focus: String, old_ship: &Ship) -> Ship {
            Ship {
                captain: new_focus,
                num_crew: old_ship.num_crew,
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct Armada {
    pub ship_1: Ship,
    pub ship_2: Ship,
}

pub mod armada {
    use crate::concrete::*;
    use crate::{Armada, Ship};

    pub struct Ship1;
    impl Lens<Ship, Ship, Armada, Armada> for Ship1 {
        fn view(&self, structure: &Armada) -> Ship {
            structure.ship_1.clone()
        }

        fn update(&self, new_focus: Ship, old_structure: &Armada) -> Armada {
            Armada {
                ship_1: new_focus,
                ship_2: old_structure.ship_2.clone(),
            }
        }
    }
}
