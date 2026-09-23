trait Vowel {
    fn make_sound(&self);
}

struct A {
    sound: String,
}

impl Vowel for A {
    fn make_sound(&self) {
        for _ in 1..5 {
            print!("{}", self.sound);
        }
        println!("\n");
    }
}

fn create_vowel(sound: String) -> impl Vowel {
    A { sound }
}

#[test]
fn trait_as_param() {
    let a = create_vowel(String::from("A"));
    let i = create_vowel(String::from("I"));

    a.make_sound();
    i.make_sound();
}
