use std::fmt::Display;

struct Container<T: Display>(Vec<T>);

impl<T: Display> Display for Container<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Container{{{}}}", self.0.iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
            .join(" | "))
    }
}

fn main() {
    let c = Container(vec![1, 2, 3]);
    println!("{}", c);
}