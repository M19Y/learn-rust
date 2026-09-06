/* String
 *
 * String is data type for string a text
 * String have 2 mode, fixed size and unfixed size it means it can grow
 *
 * For declaring a string fixed size -> let name: &str = "Max"
 * For declaring a string unfixed size -> let name: String = String::from("Verstapen");
 *
 */

fn main(){
    let first_name: &str = "Elon";
    let last_name: String = String::from("Musk");

    println!("The owner of space X is {} {}", first_name, last_name);
}

#[test]
fn immutable_string(){
    /* &str is immutable, we cannot change the value
     * &str is stored in STACK MEMORY, that means its fixed
     * primitive data type is always stored in STACK MEMORY, beacuse they are fixed
     */

    let name: &str = "Jokowi";
    let president_number: u8 = 7;
    // both variables above are stored in STACK MEMORY
    println!("The {}th President of Indonesia is {}", president_number, name);

    let status: String = String::from("Worse");
    println!("{} is one of the {} President in the Indonesia history", name, status);  
}

#[test]
fn mutable_string(){
    /* We already know the &str is fixed, we cannot change it
     * but if we try to change it using mutable variables
     * it dosent means the value in memory will change
     * it will create a new location in the stack memory
     * the oldest one its still reminding in stack memory
     *
     * However, when we use String. We can change its value in the memory
     * u must pay attention, some of the string function behave differently
     * some of them will also create a new String in the Heap memory.
     * Please Read the official documentation, before you use them.
     */

    let mut mood: &str = "Happy";
    println!("She is very {} today", mood);

    mood = "Angry";
    println!("But sometime She is {}", mood);

    // We modify string using push_str, we'll not create a new string in heap memory
    let mut description: String = String::from("one of the");
    println!("Indonesia {}", description);
    description.push_str(" most beautiful country");
    println!("Indonesia {}", description);

    let _ = description.replace("beautiful", "visited");
    // the value is not change!
    println!("Malaysia {}", description);

    // Replace function it will create a new string in heap memory
    let replace: String = description.replace("beautiful", "visited");
    println!("Malaysia {}", replace);


}
