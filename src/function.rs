/* Function
*
*
*/

fn main() {
    say_hello();

    introduction("Max", 26);

    let factorial: u32 = factorial(5);

    println!("{}", factorial);

    print_text_recursive("Jokowi", 7);

    let fac_recursive: u32 = factorial_recursive(5);

    println!("{}", fac_recursive);
}

fn introduction(name: &str, age: u8) {
    println!("My name is {}, and i am {} years old", name, age);
}

fn say_hello() {
    println!("Hai");
}

fn factorial(n: u32) -> u32 {
    if n < 1 {
        return 0;
    }

    let mut result = 1;

    for i in 1..=n {
        result = result * i;
    }

    result // the last line should be return automaticaly
}

fn print_text_recursive(value: &str, times: u8) {
    if times == 0 {
        return;
    } else {
        println!("{} - {}", value, times)
    }

    print_text_recursive(value, times - 1);
}

fn factorial_recursive(n: u32) -> u32 {
    let var_name = if n < 1 {
        return 1;
    };

    n * factorial_recursive(n - 1)
}
