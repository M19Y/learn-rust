mod first;
mod second;

use first::f_move as first_f_move;
use first::initial as first_initial;
use second::ability;
use second::f_move;
use second::initial as second_initial;

fn main() {
    // first
    first_f_move();
    first_initial();

    // second
    second_initial();
    f_move();
    ability();

    // nested first
    first::third::fouth::swim();
}
