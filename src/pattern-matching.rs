/* Pattern Matching
*
*
*/

enum Level {
    Regular,
    Premium,
    Platinum,
}

fn matching_level(level: Level) {
    match level {
        Level::Regular => {
            println!("Level Regular");
        }
        Level::Premium => {
            println!("Level Premium");
        }
        Level::Platinum => {
            println!("Level Platinum");
        }
    }
}

fn main() {
    let level1: Level = Level::Premium;
    let level2: Level = Level::Regular;
    let level3: Level = Level::Platinum;

    matching_level(level1);
    matching_level(level2);
    matching_level(level3);
}

// enum data
enum Payment {
    // card number
    CreditCard(String),

    // bank name, account number
    BankTransfer(String, String),

    // e-wallet name, e-wallet number
    EWallet(String, String),
}

impl Payment {
    fn pay(&self, amount: u32) {
        match self {
            Payment::CreditCard(number) => {
                println!("Paying with credit card {} amount {}", number, amount);
            }
            Payment::BankTransfer(bank, number) => {
                println!(
                    "Paying with bank transfer via {}-{} with amount {}",
                    bank, number, amount
                );
            }
            Payment::EWallet(e_wallet, number) => {
                println!(
                    "Paying with e-wallet with {}-{} amount {}",
                    e_wallet, number, amount
                );
            }
        }
    }
}

#[test]
fn destructing_enum_data_patterns() {
    let credit_card: Payment = Payment::CreditCard(String::from("12345C"));
    let bank_transfer: Payment =
        Payment::BankTransfer(String::from("BRI"), String::from("BRI12345"));
    let e_wallet: Payment = Payment::EWallet(String::from("Dana"), String::from("Dana12345"));

    credit_card.pay(10_000);
    bank_transfer.pay(1000_000);
    e_wallet.pay(100_000_000);
}

fn is_vowel(value: &str) {
    match value {
        "A" | "I" | "U" | "E" | "O" => {
            println!("{} is a vowel", value);
        }
        other => {
            println!("{} is not a vowel", other);
        }
    }
}

#[test]
fn pattern_matching_for_value() {
    let e_vowel: &str = "E";
    let z_isnt_vowel: &str = "Z";
    is_vowel(e_vowel);
    is_vowel(z_isnt_vowel);
}

fn exam_score(value: &u8) {
    match value {
        80..=100 => {
            println!("Amzing");
        }
        70..=79 => {
            println!("Good");
        }
        60..=69 => {
            println!("Not Bad");
        }
        50..=59 => {
            println!("Bad");
        }
        0..=49 => {
            println!("Unbelivabel");
        }
        other => {
            println!("Crazy {}", other);
        }
    }
}

#[test]
fn range_patters() {
    let my_score: u8 = 70;
    let rahul: u8 = 101;
    let his_score: u8 = 65;
    let her_score: u8 = 84;
    let prabowo_score: u8 = 11;

    exam_score(&my_score);
    exam_score(&rahul);
    exam_score(&his_score);
    exam_score(&her_score);
    exam_score(&prabowo_score);
}

// Struct Tuple
struct GeoPoint(f64, f64);

fn print_geo_point(geo_point: &GeoPoint) {
    match geo_point {
        GeoPoint(long, 0.0) => {
            println!("long: {}", long);
        }
        GeoPoint(0.0, lat) => {
            println!("lat: {}", lat);
        }
        GeoPoint(long, lat) => {
            println!("long: {}, lat: {}", long, lat);
        }
    }
}

// we also can ignoring some field
fn print_geo_point_ignore(geo_point: &GeoPoint) {
    match geo_point {
        GeoPoint(_, lat) => {
            println!("long: ignore, lat: {}", lat);
        }
    }
}

#[test]
fn destructuring_struct_patterns() {
    let point1: GeoPoint = GeoPoint(1.1, 0.0);
    let point2: GeoPoint = GeoPoint(0.0, 2.2);
    let point3: GeoPoint = GeoPoint(1.1, 2.2);

    print_geo_point(&point3);
    print_geo_point(&point2);
    print_geo_point(&point1);
    print_geo_point_ignore(&point3);
}

struct Person {
    first_name: String,
    middle_name: String,
    last_name: String,
    age: u8,
}

fn print_person(person: &Person) {
    match person {
        Person {
            first_name,
            last_name,
            ..
        } => {
            println!("First name: {}, Last name: {}", first_name, last_name);
        }
    }
}

#[test]
fn destructuring_struct_patterns_2() {
    let first_name: String = String::from("Elon");
    let last_name: String = String::from("Musk");

    let elon: Person = Person {
        first_name,
        middle_name: String::from("Holand"),
        last_name,
        age: 27,
    };

    print_person(&elon);

    println!("first name = {}", elon.first_name);
    println!("middle name = {}", elon.middle_name);
    println!("last name= {}", elon.last_name);
    println!("age = {}", elon.age);
}
