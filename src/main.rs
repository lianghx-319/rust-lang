fn main() {
    let x = 5;
    println!("The value of x is {}", x);

    let x = x + 1;
    let x = x * 2;
    println!("The value of x is {}", x);

    const MAX_POINTS: u32 = 100_000;
    println!("MAX_POINTS is {}", MAX_POINTS);

    let spaces = "     ";
    let spaces = spaces.len();

    println!("{}", spaces);
}
