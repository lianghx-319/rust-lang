fn main() {
    let s = String::from("Hello world");

    let hello = &s[..5];
    let world = &s[6..];
    let whole = &s[..];

    println!("{}, {}, {}", hello, world, whole);
}
