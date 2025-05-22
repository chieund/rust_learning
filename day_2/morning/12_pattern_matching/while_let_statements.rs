fn main() {
    let mut name = String::from("Comprehensive Rust");
    while let Some(c) = name.pop() {
        dbg!(c);
    }
}