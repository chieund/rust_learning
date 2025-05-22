#[rustfmt::skip]
fn main() {
    let input = 'x';
    match input {
        'q' => println!("Quitting"),
        'a' | 's' | 'w' | 'e' => println!("Moving around letter"),
        '0' .. '9' => println!("Number input"),
        key if key.is_lowercase() => println!("Lowercase key: {key}"),
        _ => println!("Invalid key"),
    }
}