mod first {
    pub fn say_hello() {
        println!("Say helo from module first")
    }
}

mod second {
    pub fn say_hello() {
        println!("Say helo from module second")
    }
}

mod third {
    pub fn say_hello() {
        println!("Say helo from module third")
    }
}

mod sixth;

// simple use
use crate::first::say_hello;

// use with alias
use second::say_hello as say_hello_second;

// use sixth::{left, right}; use several function in the method

// used all method in that module
use sixth::*;

fn main() {
    say_hello();
    say_hello_second();
    third::say_hello(); // call via module

    left();
    right();
    move_backward();
    move_forward();
    jump();
}
