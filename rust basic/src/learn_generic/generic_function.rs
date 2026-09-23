fn min<T: PartialOrd>(a: T, b: T) -> T {
    if a < b { a } else { b }
}

fn main() {
    println!("{}", min::<u8>(9, 11));
    println!("{}", min(2012, 2001)); // by default it used i32
}
