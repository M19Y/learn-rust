/* String Slice
*
*
*/

fn main() {
    let name: String = String::from("Max Verstapen");

    let first_name: &str = &name[0..3];
    println!("first name = {}", first_name);

    let last_name: &str = &name[4..];
    println!("last name = {}", last_name);
}
