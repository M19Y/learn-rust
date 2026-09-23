struct Foo<T> {
    value: T,
}

impl<T> Foo<T> {
    fn get_value(&self) -> &T {
        &self.value
    }
}

fn main() {
    let foo: Foo<String> = Foo::<String> {
        value: String::from("Foo Value"),
    };

    println!("foo: {}", foo.get_value());

    // inverse
    let foo1 = Foo { value: 100 };
    println!("foo1: {}", foo1.get_value());
}
