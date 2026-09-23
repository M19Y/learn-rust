trait TFooBar {
    fn get_value(&self) -> &String;
}

struct Foo {
    value: String,
}

impl TFooBar for Foo {
    fn get_value(&self) -> &String {
        &self.value
    }
}

struct Bar {
    value: String,
}

impl TFooBar for Bar {
    fn get_value(&self) -> &String {
        &self.value
    }
}

struct FooBar<T: TFooBar = Foo> {
    foo_bar: T,
}

fn main() {
    let foo = Foo {
        value: String::from("My foo"),
    };

    let bar = Bar {
        value: String::from("My bar"),
    };

    let foo_bar1 = FooBar { foo_bar: foo };
    let foo_bar2 = FooBar { foo_bar: bar };

    println!("foobar1 = {}", foo_bar1.foo_bar.get_value());
    println!("foobar2 = {}", foo_bar2.foo_bar.get_value());
}
