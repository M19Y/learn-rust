/* Tuple
 * 
 * Tuple is group of data type that have more than 1 data type
 * The amount of data in tuple is final. it's means the data type neither decreasing nor increasing
 * When we create 3 data inside tuple it means, we only have 3 data type, and we cannot change it.
 *
 */

fn main(){
    let data: (i32, f64, bool) = (9, 11.9, true);
    println!("{:?}", data);
}
 
#[test]
fn immutable_tuple(){
    let data: (i8, bool, char) = (2, false, 'G');
    
    let a = data.0;
    let b = data.1;
    let c = data.2;

    println!("a = {}, b = {}, c = {}", a, b, c);

    // we cannot change the value, because its immutable
    // data.0 = 9;
}

#[test]
fn mutable_tuple(){
    let mut data: (i8, bool, char) = (2, false, 'G');
    
    let a = data.0;
    let b = data.1;
    let c = data.2;

    println!("a = {}, b = {}, c = {}", a, b, c);

    // we can change the value
    data.0 = 9;
    data.1 = true;
    data.2 = 'W';

    // its hard to call the data, we should type it manually like data.0 data.1 ... data.n
    println!("first = {}, second = {}, third = {}", data.0, data.1, data.2);

    // the solution is 'destructuring'
    let (a, b, c) = data;

    println!("a = {}, b = {}, c = {}", a, b, c);

    // if u dont want to use one of them u should type '_', its mean i dont want to use it
    let (x, y, _) = data;

    println!("x = {}, y = {}", x, y);
}

fn empty_unit_function(){
    // this is unit function, the return of this function is empty tuple
    println!("This is an empty function that return empty tuple");
}

#[test]
fn empty_tuple(){
    let empty_result: () = empty_unit_function();
    println!("{:?}", empty_result);

    let empty_tuple: () = ();
    println!("{:?}", empty_tuple);
}   
