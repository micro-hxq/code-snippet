use std::fmt::{Debug, Display};

fn format<T: Debug + Display, const N: usize>(arr: [T; N]) -> String {
    arr.into_iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(", ")
}

fn main() {
    let arr1 = [1, 2, 3];
    let arr2 = ["a", "b", "c", "d"];
    let arr3 = [1.1, 2.2, 3.3, 4.4, 5.5];
    assert_eq!(format(arr1), "1, 2, 3");
    assert_eq!(format(arr2), "a, b, c, d");
    assert_eq!(format(arr3), "1.1, 2.2, 3.3, 4.4, 5.5");
}