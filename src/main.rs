fn main() {
    let s = String::from("Hello world");
    let work_index = first_world(&s);

    println!("{}", work_index);
}

fn first_world(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}
