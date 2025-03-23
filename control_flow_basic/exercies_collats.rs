fn collatz_length(mut n: u32) -> u32 {
    let mut length = 1;
    while n != 1 {
        if n % 2 == 0 {
            n /= 2
        } else {
           n = 3 * n + 1
        }
        length += 1
    }
    length
}


fn main() {
    println!("Length: {}", collatz_length(10));
}