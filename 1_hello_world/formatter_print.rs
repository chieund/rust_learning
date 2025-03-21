fn main() {
    println!("{} day", 31);

    println!("{0}, this is {1}, {1}, this is {0}", "Alice", "Bod");

    println!("{subject} {verb} {object}", 
        object="the lazy dog",
        subject="the quick brown fox",
        verb="jumps over"
    );

    println!("base 10:      {}", 69420); // 69420
    println!("base 2 (binary):  {:b}", 69420); // 10000111100101100
    println!("base 8 (octal):   {:o}", 69420); // 207454
    println!("base 10 (hexadecimal): {:x}", 69420); // 10f2c

    println!("{number:>5}", number = 1);
    println!("{number:0>5}", number = 1);
    println!("{number:0<5}", number = 1);
    println!("{number:0>width$}", number =1, width = 5);

    println!("My name is {0}, {1}, {0}", "Bond", 1);

    #[allow(dead_code)]
    struct Structure(i32);

    let number: f64 = 1.0;
    let width: usize = 5;
    println!("{number:>width$}");
}