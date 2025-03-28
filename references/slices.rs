fn main() {
    let a: [i32; 6] = [10, 20, 30, 40, 50, 60];
    println!("a: {a:?}");

    let s: &[i32] = &a[2..4];
    println!("s: {s:?}");

    // slice first to index
    println!("=================");
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let s: &[i32] = &a[..3];
    println!("s: {:?}", s);

    // slice from index to end
    println!("=================");
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let s: &[i32] = &a[2..];
    println!("s: {:?}", s);

    println!("=================");
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let s: &[i32] = &a[..];
    println!("s: {:?}", s);
}