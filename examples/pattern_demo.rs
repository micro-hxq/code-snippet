use std::any::type_name_of_val;

#[derive(Debug)]
struct Foo {
    name: String,
}

fn some_fun_with_ref_param(Foo { name }: &Foo) {
    // name refers to Foo.name
    println!("{}", type_name_of_val(&name))
}

fn some_fun_with_param(Foo { name }: Foo) {
    // name takes ownership of Foo.name
    println!("{}", type_name_of_val(&name));
}

fn main() {
    let foo = Foo {
        name: "hxq".to_string(),
    };
    some_fun_with_ref_param(&foo);
    some_fun_with_param(foo);
    // foo has been moved, below line will cause error
    // println!("{:?}", foo);
}