/* Sequences
*
* Sequences is dataype of the collection its like an array, its also have index
* Rust provide several Sequences data type
* 1. Vec (Vector)
* 2. VecDeque
* 3. LinkedList
*
*/

fn main() {
    println!("Sequences Collection");
}

// 1. Vec (LIFO)
#[test]
fn vector_last_in_first_out() {
    let mut names = Vec::<String>::new();
    names.push(String::from("Otong"));
    names.push(String::from("Surotong"));
    names.push(String::from("Markotong"));

    for name in &names {
        // names have to be reference if we want to used it agein
        print!("{} ", name);
    }

    print!("\n");

    println!("{:?}", names);
}

// 2. VecDeque (FIFO)
use std::collections::VecDeque;
#[test]
fn verctor_deque_first_in_first_out() {
    let mut fruits = VecDeque::<String>::new();
    fruits.push_back(String::from("Mango"));
    fruits.push_back(String::from("Strowbery"));
    fruits.push_front(String::from("Grape"));

    for fruit in &fruits {
        println!("{}", fruit);
    }

    println!("{}", fruits[0]);
}

// 3. LinkedList
use std::collections::LinkedList;
#[test]
fn verctor_linkedlist_first_in_first_out() {
    let mut fruits = LinkedList::<String>::new();
    fruits.push_back(String::from("Mango"));
    fruits.push_back(String::from("Strowbery"));
    fruits.push_front(String::from("Grape"));

    for fruit in &fruits {
        println!("{}", fruit);
    }

    // we cannot do this in LinkedList collections
    // println!("{}", fruits[0]);
}
