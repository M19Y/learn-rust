mod model {
    pub struct User {
        pub name: String,
        pub age: u8,
    }

    impl User {
        pub fn say_hello(&self, name: &str) {
            println!(
                "Hello {}, my name is {} and i'm {} years old, nice to see you",
                name, self.name, self.age
            );
        }
    }
}

fn main() {
    let user1: model::User = model::User {
        name: String::from("ucup"),
        age: 26,
    };

    user1.say_hello("Udin");
    println!();
}
