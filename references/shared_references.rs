fn main() {
    let a = 'A';
    let b = 'B';

    let mut r: &char = &a;
    dbg!(*r);

    r = &b;
    dbg!(*r);
}