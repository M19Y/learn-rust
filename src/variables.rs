fn main(){
    let name: &str = "Super Hero"; // immutable variable (we cannot change the value)
    let mut age: i32 = 24; // mutable variable (we can change the values)

    println!("Hello {}", name);
    println!("Your age is {} years old", age);
    age += 1;
    println!("Next years your age should be {} years old", age);
}


#[test]
fn immutable_variable(){
    // We can not change value inside immutable variable
    let name = "Carlos Sains";

    // name = "Max Verstapen"; // it will trhow an error immediatly
    
    println!("Hello {}", name);
}

#[test]
fn mutable_variable(){
    // We can change value inside mutable variable
    let mut age: i32 = 24; 

    println!("I'am {} years old", age);
   
    age += 1;
    println!("I'am {} years old next years", age);
}

#[test]
fn shadowing(){
    // in rust we have ability to stack the variable, its called shadowing

    let happy = "Max Verstapen"; 
    println!("I am {} the F1 Drivers", happy);
    
    let happy = 5;
    println!("I have {} worlds title in F1", happy);
}
