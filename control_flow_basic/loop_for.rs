fn main() {

    // while
    let mut x = 200;
    while x >= 10 {
        x  = x / 2;
    };
    dbg!(x);

    // for
    for x in 1..5 {
        dbg!(x);
    }

    for elem in [2, 4, 8, 16, 32] {
        dbg!(elem);
    }

    // loop
    let mut i = 0;
    loop {
        i += 1;
        dbg!(i);
        if i > 100 {
            break;
        }
    }
}