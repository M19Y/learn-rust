fn main() {
    let sum: fn(i32, i32) -> i32 = |value1: i32, value2: i32| -> i32 { value1 + value2 };

    println!("1 + 1 = {}", sum(1, 1));
}

fn print_with_filter(value: String, filter: fn(String) -> String) {
    let result = filter(value);
    println!("Result = {}", result);
}

#[test]
fn closure_as_parameter() {
    let name = String::from("Otong Surotong");
    print_with_filter(name.clone(), |value: String| -> String {
        value.to_uppercase()
    });

    let filter = |value: String| -> String { value.to_lowercase() };
    print_with_filter(name.clone(), filter);
}

fn to_uppercase(value: String) -> String {
    value.to_uppercase()
}

#[test]
fn called_closure_from_exsisting_function() {
    let name = String::from("Otong Surotong");

    // we can called existing function
    // we don't need to used the parentesis ()
    // we just called its name
    print_with_filter(name, to_uppercase);
}

#[test]
fn closure_scope() {
    let mut counter = 1;

    let mut increment = || {
        // by doing this, we can change other 'counter' variable
        counter += 1;
        println!("Increment");
    };

    increment();
    increment();
    increment();

    println!("Counter : {}", counter);
}

struct Counter {
    count: u32,
}

impl Counter {
    fn increment(&mut self) {
        self.count += 1;
        println!("Increment");
    }
}

#[test]
fn alternative_counter_with_struct() {
    let mut counter = Counter { count: 0 };
    counter.increment();
    counter.increment();
    counter.increment();

    println!("Counter : {}", counter.count);
}
