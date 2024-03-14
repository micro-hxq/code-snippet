#[derive(Debug)]
struct Foo<'a> {
    x: &'a i32,
}


impl<'a> Foo<'a> {
    fn new(x: &'a i32) -> Self {
        Foo { x }
    }

    fn largest_x<'b>(&self, bx: &'b i32) -> &'b i32
        where 'a : 'b {
        if self.x > bx {
            self.x
        } else {
            bx
        }
    }
}


fn main() {
    let x = 10;
    let foo = Foo::new(&x);
    let bx = 20;
    println!("largest_x: {}", foo.largest_x(&bx));
}