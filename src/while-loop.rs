/*While loop
 *
 */

fn main(){
    let mut counter: u8 = 0;

    while counter <= 10 {

        if counter % 2 == 0 {
            println!("Counter = {}", counter);
        }
        
        counter += 1;
    }
}
