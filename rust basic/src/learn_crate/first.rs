pub fn initial() {
    println!("I am the first");
}

pub fn f_move() {
    println!("Fly");
}

pub mod third {
    pub mod fouth {
        pub fn swim() {
            println!("i am swiming");
            // access the initial fn from its parent
            // 1. used crate
            crate::first::initial();
            // 2. used super keyword
            super::super::f_move()
        }
    }
}
