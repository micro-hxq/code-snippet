fn main() {
    for i in 1..=5 {
        match i {
            // without (), the equivalent pattern is (num @ 1) | 2 => ...
            num @ (1 | 2) => println!("the first branch is {}", num),
            // before rust 1.53
            num @ 3 | num @ 4 => println!("the second branch is {}", num),
            _ => println!("the default branch")
        }
    }
}