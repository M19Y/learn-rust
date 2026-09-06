/* Function Ownership
 *
 * Ownership in function parameter behave same as other Ownership.
 * When the parameter is stored in STACK MEMORY it will just copy its value
 * and it will create new allocation memory in STACK MEMORY.
 * When the parameter is stored in HEAP MEMORY it will give its value and its Ownership
 * to the parameter in that function, the value and ownership will be vanish when the function
 * compilte it task
 *
 */

fn stack_memory_params(text: &str) {
    println!("My name is {}, and i'll stored in STACK MEMORY", text);
}

fn heap_memory_params(text: String) {
    println!("Hai {}, you will stored in HEAP MEMORY", text);
}

#[test]
fn stack_memory() {
    let name: &str = "Max";
    stack_memory_params(name);
    println!("Call {} again!", name);

    /* The flow goes like this:
     * 'name' set "Max"
     * stack_memory_params need a param for 'text' param
     * 'name' is passing to the stack_memory_params
     * the 'text' param is copying the value of 'name'
     * the have different ownership, because they have complitly different ownership
     * the value of 'name' is still alive
     * so we can call 'name' again, even though stack_memory_params already called
     */
}

#[test]
fn heap_memory() {
    let name: String = String::from("Verstapen");
    heap_memory_params(name);
    // println!("Call {} again!", name); <--- we cannot call the name age, because the value already
    // vanish in param name inside heap_memory_params

    /* The fow goes like this:
     * 'name' set "Verstapen"
     * heap_memory_params need a param for 'text' param
     * 'name is passing to the heap_memory_params
     * the 'text' param now is the ownership of "Verstapen"
     * 'name' now have not value and ownership anymore.
     * Remember the rules!
     * in one time there is only one owner, you can't have two, three or more
     * after heap_memory_params called, 'name' value is vanish also 'text'
     * so we cannot called it again after they all vanish
     */
}

fn full_name(first_name: String, last_name: String) -> String {
    format!("{} {}", first_name, last_name)
}

#[test]
fn return_value_function_ownership() {
    let first_name: String = String::from("Elon");
    let last_name: String = String::from("Musk");

    let full_name: String = full_name(first_name, last_name);

    println!("Fullname: {}", full_name);

    // we cannot called the first_name and last_name. because its ownership already move into full_name
    // println!("{}", first_name);
    // println!("{}", last_name);
}

fn return_back_ownership(first: String, last: String) -> (String, String, String) {
    let full = format!("{} {}", first, last);
    (first, last, full)
}

#[test]
fn retur_back_ownership_tuple() {
    let first = String::from("Mango");
    let last = String::from("Sweet");
    let (first, last, full) = return_back_ownership(first, last);

    println!("first: {}", first);
    println!("last: {}", last);
    println!("full: {}", full);
}
