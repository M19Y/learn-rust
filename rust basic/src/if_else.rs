/* If Else condition
 *
 *
 */

fn main(){

    let score: u8 = 7;

    if score >= 9 {
        println!("Amazing");
    }else if score >= 7 {
        println!("Good");
    }else if score >= 5 {
        println!("Not good");
    }else{
        println!("You should learn more often");
    }
}

#[test]
fn let_statement(){

    // assing the result to other variables 
    let score: u8 = 7;
    let result: &str;

    if score >= 9 {
        result = "Amazing";
    }else if score >= 7 {
        result = "Good";
    }else if score >= 5 {
        result = "Not Good ";
    }else{
        result = "You should learn more often";
    }

    println!("{}", result);

    // assing the result directly to let statment
    let point: u8 = 75;
    let mood: &str = if point >= 70 { "Happy" } else {"Sad"};
    println!("The current mood is {}", mood);
}
