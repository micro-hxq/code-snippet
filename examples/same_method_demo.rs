struct Foo;

trait Bar {
    fn fun1();
    fn fun2(&self);
}

trait Zoo {
    fn fun2(&self);
}

impl Foo {
    fn fun1() {
        println!("Foo fun1");
    }

    fn fun2(&self) {
        println!("Foo fun2");
    }
}

impl Bar for Foo {
    fn fun1() {
        println!("Bar fun1");
    }

    fn fun2(&self) {
        println!("Bar fun2");
    }
}

impl Zoo for Foo {
    fn fun2(&self) {
        println!("Zoo fun2");
    }
}

fn main() {
    let foo = Foo;
    Foo::fun1();
    // type method priority is higher than trait method
    foo.fun2();

    // assciate method must use full qualified syntax
    // Bar::fun1();
    Bar::fun2(&foo);
    Zoo::fun2(&foo);

    println!("full qualified syntax:");
    <Foo as Bar>::fun1();
    <Foo as Bar>::fun2(&foo);
    <Foo as Zoo>::fun2(&foo);
}