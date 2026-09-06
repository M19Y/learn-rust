/* Method
*
*
*
*/

struct Person {
    first_name: String,
    last_name: String,
    age: u8,
}

impl Person {
    // associated function
    fn create(first_name: &str, last_name: &str, age: u8) -> Person {
        Person {
            first_name: String::from(first_name),
            last_name: String::from(last_name),
            age,
        }
    }

    // associated method
    fn say_hello(&self, name: &str) {
        println!(
            "Hello {}, my name is {} {}, and i am {} years old",
            name, self.first_name, self.last_name, self.age
        );
    }
}

fn main() {
    let elon: Person = Person {
        first_name: String::from("Elon"),
        last_name: String::from("Musk"),
        age: 35,
    };

    let max: Person = Person::create("Max", "Verstapen", 25);

    elon.say_hello(&max.first_name);
}
