struct Student<'b, 'y> {
    name: &'b str,
    last_name: &'y str,
}

impl<'x, 'z> Student<'x, 'z> {
    fn longest_name(&self, student: &Student<'x, 'z>) -> &'x str {
        if self.name.len() > student.name.len() {
            self.name
        } else {
            student.name
        }
    }
}

fn main() {
    let student1 = Student {
        name: "John",
        last_name: "n",
    };
    let student2 = Student {
        name: "Dea",
        last_name: "a",
    };
    println!(
        "Student has longest name is {}",
        student1.longest_name(&student2)
    );
}
