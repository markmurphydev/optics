trait One<A> {
    fn print_one(&self) {
        println!("ONE")
    }
}

impl<F, A> One<A> for F where F: Fn(A) -> A {}

trait Two<A, B> {
    fn print_two(&self) {
        println!("TWO")
    }
}

impl<F, A, B> Two<A, B> for F where F: Fn(A) -> B {}

fn id<A>(a: A) -> A {
    a
}

fn return_one<A>() -> impl One<A> {
    id
}

fn return_two<A>() -> impl Two<A, A> {
    id
}
