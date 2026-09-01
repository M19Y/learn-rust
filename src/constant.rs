/* Constant
 *
 * Constant is a way to create variable that we must 
 * - final
 * - unchange
 * - must declared before its run
 */

const MAXIMUM: u8 = 100;

fn main(){
    const MINIMUM: i8 = 1;
    
    println!("Maximum = {}, Minimum = {}", MAXIMUM, MINIMUM);
}
