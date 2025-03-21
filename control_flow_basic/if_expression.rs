fn main() {
    let x = 10;
    if x == 0 {
        println!("zero!");
    } else if x < 100 {
        println!("biggish");
    } else {
        println!("huge");
    }

    let size = if x < 20 { "small" } else { "large" };
    println!("number size: {}", size);
}