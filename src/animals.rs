use std::marker::PhantomData;

struct IsAnimal;
struct IsDog;

trait Animal: Sized {
    type AnimalClass;

    fn do_animal_thing(&self, text: String);

    fn with_animal<T: Animal>(
        self,
        other: T,
    ) -> TwoAnimals<Self, T, Self::AnimalClass, T::AnimalClass> {
        TwoAnimals {
            a: self,
            b: other,
            a_class: PhantomData,
            b_class: PhantomData,
        }
    }
}

trait Dog: Sized {
    fn bark(&self, text: String);
}

impl<U: Dog> Animal for U {
    type AnimalClass = IsDog;

    fn do_animal_thing(&self, text: String) {
        self.bark(text);
    }
}

struct TwoAnimals<A, B, AClass, BClass> {
    a: A,
    b: B,
    a_class: PhantomData<AClass>,
    b_class: PhantomData<BClass>,
}

impl<A: Animal, B: Animal> Animal for TwoAnimals<A, B, IsAnimal, IsAnimal> {
    type AnimalClass = IsAnimal;

    fn do_animal_thing(&self, text: String) {
        self.a.do_animal_thing(text.clone());
        self.b.do_animal_thing(text);
    }
}

impl<A: Animal, B: Animal> Animal for TwoAnimals<A, B, IsAnimal, IsDog> {
    type AnimalClass = IsAnimal;

    fn do_animal_thing(&self, text: String) {
        self.a.do_animal_thing(text.clone());
        self.b.do_animal_thing(text);
    }
}

impl<A: Animal, B: Animal> Animal for TwoAnimals<A, B, IsDog, IsAnimal> {
    type AnimalClass = IsAnimal;

    fn do_animal_thing(&self, text: String) {
        self.a.do_animal_thing(text.clone());
        self.b.do_animal_thing(text);
    }
}

impl<A: Dog, B: Dog> Dog for TwoAnimals<A, B, IsDog, IsDog> {
    fn bark(&self, text: String) {
        self.a.bark(text.clone());
        self.b.bark(text);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn make_2_animals() {
        struct MyKoala;
        impl Animal for MyKoala {
            type AnimalClass = IsAnimal;

            fn do_animal_thing(&self, text: String) {
                println!("Koala noises: {text}");
            }
        }
        struct MyDog;
        impl Dog for MyDog {
            fn bark(&self, text: String) {
                println!("Bark: {text}");
            }
        }

        let koala = MyKoala;
        let poodle = MyDog;
        let koala_poodle = koala.with_animal(poodle);
        koala_poodle.do_animal_thing("koala_poodle".to_string());

        let lab = MyDog;
        let shihtzu = MyDog;
        let lab_shihtzu = lab.with_animal(shihtzu);
        lab_shihtzu.bark("lab_shihtzu".to_ascii_lowercase());
    }
}
