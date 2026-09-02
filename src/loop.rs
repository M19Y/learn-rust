/* Loop
 *
 * Loop will do repeatable work until it reach breaking pont to stop
 * to break a loop we sould break with 'break' key word
 *
 */

fn main(){

    let mut counter: u8 = 0;

    loop {

        counter = counter + 1;

        if counter > 10 {
            break;
        } else if counter % 2 == 0 {
            continue;
        }

        println!("Counter = {}", counter);
    }

}

#[test]
fn break_as_return_value(){

    let mut counter: u8 = 0;

    let result: u8 = loop {
        counter += 1;
        if counter > 10 { break counter * 2 }
    };

    println!("Result = {}", result);
    assert_eq!(result, 22);
    assert_eq!(counter, 11);
}

#[test]
fn loop_label(){
    // we can break a loop by its lable

    let mut counter: u8 = 1;

    'outer:loop {

        let mut num: u8 = 1;


        loop {

            if counter > 10 { break 'outer }

            println!("{} x {} = {}", counter, num, counter * num);

            num += 1;
            if num > 10 { break }
        }

        println!("=================================\n");

        counter += 1;

    }

}
