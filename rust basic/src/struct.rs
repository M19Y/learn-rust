/* Struct
*
*/
struct Person {
    first_name: String,
    middle_name: String,
    last_name: String,
    age: u8,
}

fn print_person(person: &Person) {
    println!("first name = {}", person.first_name);
    println!("middle name = {}", person.middle_name);
    println!("last name= {}", person.last_name);
    println!("age = {}", person.age);
}

// Struct Tuple
struct GeoPoint(f64, f64);

// Empty Struct
struct Nothing;

fn main() {
    // init shorthand
    let first_name: String = String::from("Elon");
    let last_name: String = String::from("Musk");

    let elon: Person = Person {
        first_name,
        middle_name: String::from("Holand"),
        last_name,
        age: 27,
    };

    print_person(&elon);

    // struct update syntax
    let elon_update: Person = Person {
        first_name: elon.first_name.clone(),
        middle_name: elon.middle_name.clone(),
        last_name: elon.last_name.clone(),
        ..elon
    };

    print_person(&elon_update);

    let geo_point = GeoPoint(-6.200000, 106.816666);
    println!("long : {}", geo_point.0);
    println!("lat : {}", geo_point.1);

    let _nothing1: Nothing = Nothing;
    let _nothing2: Nothing = Nothing {};
}
