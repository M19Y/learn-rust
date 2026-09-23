use std::fmt::Display;

enum Value<T> {
    NONE,
    PRESENT(T),
}

fn check_value<T: Display>(value: Value<T>) {
    match value {
        Value::NONE => {
            println!("none");
        }
        Value::PRESENT(n) => {
            println!("The value of present is {}", n);
        }
    }
}

fn main() {
    let value: Value<u8> = Value::<u8>::PRESENT(10);
    check_value(value);
    /*
        match value {
            Value::NONE => {
                println!("none");
            }
            Value::PRESENT(n) => {
                println!("The value of present is {}", n);
            }
        }
    */
}
