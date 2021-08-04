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

    let guess: u32 = "42".parse().expect("Not a number");
    println!("{}", guess);

    // number
    let decimal = 98_222;
    let hex = 0xff;
    let octal = 0o77;
    let binary = 0b1111_0000;
    let byte = b'B';
    println!("{}, {}, {}, {}, {}", decimal, hex, octal, binary, byte);
}
