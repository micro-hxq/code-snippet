fn main() {
    // string format
    let name = "hxq";
    assert_eq!(format!("Hello, {name}!"), "Hello, hxq!");
    assert_eq!(format!("Hello, {name:.2}!"), "Hello, hx!");
    assert_eq!(format!("Hello, {name:5}!"), "Hello, hxq  !");
    assert_eq!(format!("Hello, {name:>5}!"), "Hello,   hxq!");
    assert_eq!(format!("Hello, {name:0<5}!"), "Hello, hxq00!");
    assert_eq!(format!("Hello, {name:&^5}!"), "Hello, &hxq&!");

    assert_eq!(format!("{{hello}} \"{name}\""), r#"{hello} "hxq""#);

    // exp
    let num = 3140000000i64;
    assert_eq!(format!("num: {num:e}"), "num: 3.14e9");
    assert_eq!(format!("num: {num:E}"), "num: 3.14E9");
}