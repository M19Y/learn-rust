trait CanSayName {
    fn say_name(&self);
}

trait CanSayGoodBye {
    fn say_good_bye(&self, to: String) -> String;
}

struct Foo {
    name: String,
}

struct Bar {
    name: String,
}

impl CanSayName for Foo {
    fn say_name(&self) {
        println!("my name is Foo {}", self.name);
    }
}

impl CanSayGoodBye for Foo {
    fn say_good_bye(&self, to: String) -> String {
        format!("bye bye {}", to)
    }
}

impl CanSayName for Bar {
    fn say_name(&self) {
        println!("my name is Bar {}", self.name);
    }
}

fn say_hello(x: &impl CanSayName) {
    println!("\nHello");
    x.say_name();
}

#[test]
fn traint_as_parameter() {
    let foo: Foo = Foo {
        name: String::from("Ucup"),
    };
    let bar: Bar = Bar {
        name: String::from("Idin"),
    };

    say_hello(&foo);
    say_hello(&bar);

    println!("\nfoo name -> {}", foo.name);
    println!("foo name -> {}", foo.name); // allow

    let bar_name: String = bar.name;
    println!("\nbar name -> {}", bar_name);
    // println!("bar name -> {}", bar.name); // not allowed
}

fn say_name_and_good_bye(x: &(impl CanSayName + CanSayGoodBye), to: String) {
    x.say_name();
    println!("{}", x.say_good_bye(to));
}

#[test]
fn multiple_trait_param() {
    let foo: Foo = Foo {
        name: String::from("Uncle Atong"),
    };
    say_name_and_good_bye(&foo, String::from("Tok Dalang"));

    /* we can not do this, because bar is not implement CanSayGoodBye
    let bar: Bar = Bar {
        name: String::from("Opah"),
    };
    say_name_and_good_bye(&bar, String::from("Uncle mutu"));
    */
}
