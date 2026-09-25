use std::fmt::{Debug, Formatter, Result};

struct Category {
    id: String,
    name: String,
}

impl Debug for Category {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("Category")
            .field("id", &self.id)
            .field("name", &self.name)
            .finish()
    }
}

fn main() {
    let category = Category {
        id: String::from("c001"),
        name: String::from("Fruits"),
    };
    println!("category -> {:?}", category);
}
