use crate::first::f_move as fly_move;

pub fn initial() {
    println!("I am the second");
}

pub fn f_move() {
    println!("Walk");
}

pub fn ability() {
    f_move();
    fly_move();
}
