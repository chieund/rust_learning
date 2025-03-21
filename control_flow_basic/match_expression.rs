fn main() {
    let val = 1;
    match val {
        1 => println!("one"),
        10 => println!("ten"),
        100 => println!("one hundred"),
        _ => {
            println!("something else")
        }
    };

    let flag = true;
    match flag {
        true => 1,
        false => 0
    };

    println!("The value of {flag} is {val}");
}