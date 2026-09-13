/* References
*
* References is a pointer that reference to value base on their location in HEAP MEMORY
* References will not affected the owner of the value its self, its just reference to their value
*
*  Borrowing
*  When reference has been made we call it borrowing, we only borrow the value.
*  by default we dont have any accessibility for changing its value
*  even though the parent variable is set mutable value, unless we also change the param
*  in that function as a mutable value '&mut'
*/

fn hai(name: &String, age: u8) -> String {
    format!("Hai my name is {}, and i am {} years old", name, age)
}

#[test]
fn string_reference() {
    let name: String = String::from("Stef");
    let age: u8 = 32;

    let result = hai(&name, age);
    println!("{}", result);
    println!("{}", name); // <- we can still call the name, cause the ownership does not change
}

/* We can't do this to immutable reference

fn invalid_change_ref(value: &String) {
    value.push_str("Invalid Change");
}

#[test]
fn invalid_change_value_reference() {
    let name = String::from("Original");
    invalid_change_ref(&name);
    println!("{}", name);
}
*/

/* mutable reference
* as we mention erlyer if we want to change its value by borrowing them,
* we must accept serveral criteria:
* - parent variable must be mutable 'let mut parent: data_type = value'
* - create function that accept mutable reference 'fn simple_function(param: &mut data_type){...}'
* - passing the value to the function 'simple_function(&mut parent)'
* - borrowing mutable reference only allowed one per one time
*   means we cannot make a reference into the same lifecycle twice
*/

#[test]
fn mutable_reference() {
    let mut skill: String = String::from("Magic");
    println!("My current skill {}", skill);

    change_skill(&mut skill);
    println!("After skill change, My current skill {}", skill);

    // we can call it multiple time, it will not const any error
    change_skill(&mut skill);
    println!("After skill change, My current skill {}", skill);

    // we allowed to to this, even though we call it twice
    // first borrow
    let skill_borrow1 = &mut skill;
    change_skill(skill_borrow1);
    println!("After skill change1, My current skill {}", skill);

    // second borrow
    let skill_borrow2 = &mut skill;
    change_skill(skill_borrow2);
    println!("After skill change2, My current skill {}", skill);

    /*
    // first borrow
    let borrow1 = &mut skill;

    // second borrow
    let borrow2 = &mut skill;

    // it will not allowed us, we want to change borrow1
    // but borrow2 also reference same as borrow1

    change_skill(borrow1);
    change_skill(borrow2);
    */
}

fn change_skill(skill: &mut String) {
    skill.push_str("Strenght")
}
