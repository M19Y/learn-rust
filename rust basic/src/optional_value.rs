use std::option::Option;

fn double(x: Option<u32>) -> Option<u32> {
    match x {
        None => None,
        Some(i) => Some(i * 2),
    }
}

fn main() {
    let empty = Option::None;
    println!("{:?}", double(empty));
    let number = Option::Some(911);
    println!("{:?}", double(number));
}
