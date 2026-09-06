/* Array
 *
 * Array is collection of data, it only have 1 data types, not like tuple!
 * to create an array it should be like this -> let array: [data_type; length]
 */

fn main(){
    let array: [i32; 5] = [11, 9, 1, 0, 2];
    println!("{:?}", array);
}

#[test]
fn immutable_array(){
    let data: [i8; 3] = [20, 40, 60];

    let a = data[0];
    let b = data[1];
    let c = data[2];

    assert_eq!(a, 20);
    assert_eq!(b, 40);
    assert_eq!(c, 60);

    println!("Data = {}, {}, {}", a, b, c);
}
 
#[test]
fn mutable_array(){
    let mut data: [f64; 3] = [26.4, 0.2, 6.3];

    let a = data[0];
    let b = data[1];
    let c = data[2];

    assert_eq!(a, 26.4);
    assert_eq!(b, 0.2);
    assert_eq!(c, 6.3);

    data[0] = 1.4;
    data[1] = 2.5;
    data[2] = 3.1;
    
    // we also can destructur the array
    let [x, y, z] = data;
    println!("Data = {}, {}, {}", x, y, z);
}

#[test]
fn array_size(){
    let vowel: [char; 5] = ['a', 'i', 'u', 'e', 'o'];
    let vowel_size: usize = vowel.len();
    println!("Vowel have {} characters and it is => {:?}", vowel_size, vowel);
}

#[test]
fn two_dimentional_array(){
    let matrix: [[i8; 3]; 2] = [
        [1, 2, 3],
        [4, 5, 6] 
    ];
    println!("{:?}", matrix);
}
