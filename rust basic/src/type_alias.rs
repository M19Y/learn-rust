/* Type Alias
*
*
*/
type Age = u8;
type IdentityNumber = String;

struct Customer {
    id: IdentityNumber,
    name: String,
    age: Age,
}

impl Customer {
    fn new(id: String, name: String, age: Age) -> Customer {
        Customer { id, name, age }
    }
}

fn main() {
    let customer1: Customer = Customer::new(
        IdentityNumber::from("cID-123"),
        String::from("Good employee"),
        25,
    );

    println!(
        "Id: {}\nName: {}\nAge: {}",
        customer1.id, customer1.name, customer1.age
    );
}
