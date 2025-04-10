struct Wrapper<F>(F);
impl<'a, F> Wrapper<F>
where
    F: Fn() + 'a,
{
    fn run(&self) {
        self.0()
    }

    fn new_wrapper(&self) -> Wrapper<impl Fn()> {
        Wrapper(|| {
            println!("New");
            self.0()
        })
    }
}

fn return_wrapper() -> Wrapper<impl Fn()> {
    Wrapper(|| println!("Wrapped!"))
}
