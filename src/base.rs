fn base() {
    // 标量类型
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

    // int
    let decimal = 98_222u32;
    let hex = 0xff;
    let octal = 0o77;
    let binary = 0b1111_0000;
    let byte = b'B';
    println!("{}, {}, {}, {}, {}", decimal, hex, octal, binary, byte);

    // float
    let x = 2.0;
    let y: f32 = 3.0;

    // operate
    let sum = 5 + 10;

    let different = 9.55 - 4.3;

    let product = 4 * 30;

    let quotient = 56.3 / 32.2;

    let reminder = 54 % 5;

    // boolean 一个字节大小
    let t = true;
    let f = false;

    // char 字符类型
    let x = 'z';
    let y: char = 'z';
    let z = '😃';

    // Tuple 元组
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("{}, {}, {}", tup.0, tup.1, tup.2);

    let a = [1, 2, 3, 4, 5];

    let months = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];

    let a = [3; 5];

    let first = months[0];
    let second = months[1];
}
