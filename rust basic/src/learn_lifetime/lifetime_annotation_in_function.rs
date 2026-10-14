/*
fn longest(value1: &str, value2: &str) -> &str {
    if value1.len() > value2.len() {
        value1
    } else {
        value2
    }
}

this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `valu
e1` or `value2`
*/

// lifetime annotation
fn longest<'m>(value1: &'m str, value2: &'m str) -> &'m str {
    if value1.len() > value2.len() {
        value1
    } else {
        value2
    }
}

fn main() {
    let longest_str = longest("first", "second");
    println!("{}", longest_str);
}

#[test]
fn dangling_reference_in_lifetime_annotation() {
    // in lifetime annotation it doesn't mean the reference will life longger than the command
    // variable behavior.

    let first = String::from("Otong");
    let result: &str;
    {
        let second = String::from("Markotong");
        result = longest(first.as_str(), second.as_str());
    }
    println!("{}", result);
}
