#[derive(Debug)]
struct Foo {}

impl Foo {
    fn immut_fun(&self) {
        println!("immut_fun parameter type: {}", std::any::type_name_of_val(&self));
    }

    fn mut_fun(&mut self) {
        println!("mut_fun parameter type: {}", std::any::type_name_of_val(&self));
    }
}


fn main() {
    let mut foo = Foo {};
    foo.mut_fun();
    // reborrowing mutable reference as immutable reference
    // equivalent to Foo::immut_fun(&*(&mut foo));
    foo.immut_fun();

    let bar = Foo {};
    bar.immut_fun();
}