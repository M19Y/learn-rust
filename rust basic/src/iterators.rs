fn main() {
    println!("Iterator");
}

#[test]
fn simple_iterator() {
    let array: [u8; 10] = [1, 2, 3, 4, 5, 10, 9, 8, 7, 6];

    // get iterator from an array
    let mut iterator = array.iter();

    while let Some(x) = iterator.next() {
        println!("{}", x);
    }

    for x in array {
        println!("{}", x);
    }
}

#[test]
fn iterator_methods() {
    // create a vector using micro
    let vector: Vec<u8> = vec![1, 2, 3, 4, 5, 10, 9, 8, 7, 6];
    println!("Original vector -> {:?}", vector);

    // sum
    let sum: u8 = vector.iter().sum();
    println!("\nSum -> {}", sum);

    // count
    let count: usize = vector.iter().count();
    println!("\nCount -> {}", count);

    // double vector
    let double_vector: Vec<u8> = vector.iter().map(|x| x * 2).collect();
    println!("\ndouble vector -> {:?}", double_vector);

    // odd
    let odd: Vec<&u8> = vector.iter().filter(|x| *x % 2 != 0).collect();

    println!("\nodd vector -> {:?}", odd);
}
