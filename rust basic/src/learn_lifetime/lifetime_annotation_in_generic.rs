struct Teacher<'a, ID>
where
    ID: Ord,
{
    id: ID,
    name: &'a str,
}

fn main() {
    let teacher: Teacher<u32> = Teacher::<u32> {
        id: 20010911,
        name: "twins",
    };

    println!("Id: {}, name: {}", teacher.id, teacher.name);
}
