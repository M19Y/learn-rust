struct Student<'b, 'y> {
    name: &'b str,
    last_name: &'y str,
}

fn longest_student_name<'a, 'g>(student1: Student<'a, 'g>, student2: Student<'a, 'g>) -> &'a str {
    if student1.name.len() > student2.name.len() {
        student1.name
    } else {
        student2.name
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
        longest_student_name(student1, student2)
    );
}
