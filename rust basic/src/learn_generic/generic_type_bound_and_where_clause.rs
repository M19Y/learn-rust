trait Hi {
    fn say_hi(&self);
}

struct Person {
    name: String,
}

impl Hi for Person {
    fn say_hi(&self) {
        println!("Hai my name is {}", self.name);
    }
}

// type bound, we shoud defined with <Type : TypeBound>
struct Wsup<T: Hi> {
    value: T,
}

// where clause
struct HeyHo<T>
where
    T: Hi,
{
    value: T,
}

fn main() {
    let introduce: Wsup<Person> = Wsup::<Person> {
        value: Person {
            name: String::from("Idin"),
        },
    };

    introduce.value.say_hi();

    let introduce1: HeyHo<Person> = HeyHo {
        value: Person {
            name: String::from("Supra"),
        },
    };

    introduce1.value.say_hi();
}
