trait GetValue<T> {
    fn get_value(&self, value: T) -> T;
}

struct Foo<T> {
    value: T,
}

impl<T> GetValue<T> for Foo<T> {
    fn get_value(&self, value: T) -> T {
        value
    }
}

struct Bar {
    value: u8,
}

impl<T> GetValue<T> for Bar {
    fn get_value(&self, value: T) -> T {
        println!("bar: {}", self.value);
        value
    }
}

fn main() {
    let foo = Foo {
        value: String::from("my foo"),
    };
    println!("foo: {}", foo.get_value(String::from("Jhonson")));
    let bar = Bar { value: 11 };
    println!("bar: {}", bar.get_value(2001));
}
