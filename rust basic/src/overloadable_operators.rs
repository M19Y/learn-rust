use std::ops::Add;

struct Apple {
    quantity: u32,
}

impl Add for Apple {
    type Output = Apple;

    fn add(self, rhs: Self) -> Self::Output {
        Apple {
            quantity: self.quantity + rhs.quantity,
        }
    }
}

fn main() {
    let apple1 = Apple { quantity: 9 };
    println!("Apple1 Quantity: {}", apple1.quantity);
    let apple2 = Apple { quantity: 11 };
    println!("Apple2 Quantity: {}", apple2.quantity);
    let apple3 = apple1 + apple2;
    println!("Apple3 Quantity: {}", apple3.quantity);
}
