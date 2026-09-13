/* Data types
 *
 * Every values in rust have their own data types
 * Rust devided their data types with 2 types
 *
 * 1. Scalar
 *    Scalar type is representing single value like integer, float, boolean, ect.
 * 2. Compound
 *    Compound type is reresenting multi values or it can be have more than one values
 *    like tuple and array
 */

fn main(){
    let name = "Lando Noris"; // implisit data type (rust can automaticaly inver this variables)
    let age: i32 = 26;

    println!("The F1 driver names is {} and his age is {} years old", name, age);
}

#[test]
fn data_types_number(){
    /*
     * Integer types
     * -------------------------------------------
     * Signed   | Unsigned   | in-bytes
     * -------------------------------------------
     * i8       | u8         | 8-bit
     * i16      | u16        | 16-bit
     * i32      | u32        | 32-bit // Default
     * i64      | u64        | 64-bit
     * i128     | u128       | 128-bit
     * -------------------------------------------
     *
     * Float types
     * -------------------------------------------
     * Signed   | in-bytes
     * -------------------------------------------
     * f32      | 32-bit 
     * f64      | 64-bit // Default
     * -------------------------------------------
     *
     * Usize types
     * -------------------------------------------
     * isize    | 32/64-bit 
     * usize    | 32/64-bit
     * -------------------------------------------
     *
     * Boolean types
     * -------------------------------------------
     * bool     | true/false
     * -------------------------------------------
     *
     * Char types
     * -------------------------------------------
     * char     | 'a' or 'b' or 'c' .. 
     * -------------------------------------------
     *
     */

    let num1: i32 = 10;
    let num2: f64 = 3.14;

    println!("num1 = {}, num2 = {}", num1, num2);
    assert_eq!(num1, 10);
    assert_eq!(num2, 3.14);
}

#[test]
fn data_types_conversion(){
    /* Be carefull if u want to convert the larges number to the smallest
     * We can use 'as' to convert data types
     */

    // Safe conversion (smallest to larges)
    let num1: i8 = 10;
    let num2: i16 = num1 as i16;

    println!("num1 = {}, num2 = {}", num1, num2);

    // Unsafe conversion (largest to smallest)
    let num3: i16 = 10_000;
    let num4: i8 = num3 as i8;

    println!("num3 = {}, num4 = {}", num3, num4);

}
