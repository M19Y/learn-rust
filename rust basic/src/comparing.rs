mod formating;

struct Apple {
    quantity: u32,
}

impl PartialEq for Apple {
    fn eq(&self, other: &Self) -> bool {
        self.quantity == other.quantity
    }
}

use std::cmp::Ordering;

impl PartialOrd for Apple {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.quantity.partial_cmp(&other.quantity)
    }
}

fn main() {
    let apple1 = Apple { quantity: 11 };
    let apple2 = Apple { quantity: 9 };

    println!(
        "Apple1: {} == Apple2: {} = {}",
        apple1.quantity,
        apple2.quantity,
        apple1 == apple2
    );
    println!(
        "Apple1: {} > Apple2: {} = {}",
        apple1.quantity,
        apple2.quantity,
        apple1 > apple2
    );
    println!(
        "Apple1: {} < {} = {}",
        apple1.quantity,
        apple2.quantity,
        apple1 < apple2
    );
}
