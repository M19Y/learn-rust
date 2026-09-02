/* For loop
 *
 *
 */

fn main(){

    // array while loop
    let vowels: [char; 5] = ['a','i','u','e','o'];

    let mut index = 0;

    println!("While loop");
    while index < vowels.len() {
        print!("{} ", vowels[index]);
        index += 1;
    }

    
    println!("\n\nFor loop");
    for i in vowels {
        print!("{} ", i);
    }

    // using range
    println!("\n\nFor loop with range");
    for i in 1 .. 5 {
        println!("loop : {}", i);
    }
}

#[test]
fn range_data_type(){
    
    let range_exclusive = 1..5;
    println!("start: {}, end: {}", range_exclusive.start, range_exclusive.end);

    for i in range_exclusive {
        print!("{} ", i);
    }

    let range_inclusive = 1..=5;
    println!("\nstart: {}, end: {}", range_inclusive.start(), range_inclusive.end());

    for i in range_inclusive {
        print!("{} ", i);
    }
}

