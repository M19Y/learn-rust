mod fifth;
mod fourth;

use fourth::say_hello;
fn main() {
    say_hello();
    fifth::say_hello();
}
