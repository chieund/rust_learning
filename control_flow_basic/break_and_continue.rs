fn main() {
    let mut i = 0;
    loop {
        i += 1;
        if i > 5 {
            break;
        }

        if i % 2 == 0 {
            continue;
        }

        dbg!(i);
    }
}