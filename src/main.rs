fn main() {
    let x: i8 = 5;
    let y: Option<i8> = None;

    match y {
        None => println!("y is not a number"),
        Some(value) => println!("{}", x + value),
    }
}
