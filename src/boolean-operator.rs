/* Boolean operator
 * And " && "
 * Or  " || "
 * Not " !  "
 *
 * && (all values must be same for resulting 'true')
 * -----------------------------------------
 * value1   | operator  | value2    | result
 * -----------------------------------------
 * true     | &&        | true      | true
 * true     | &&        | false     | false
 * false    | &&        | true      | false
 * false    | &&        | false     | false
 * -----------------------------------------
 *
 * || (At least one value true, the result must be true)
 * -----------------------------------------
 * value1   | operator  | value2    | result
 * -----------------------------------------
 * true     | ||        | true      | true
 * true     | ||        | false     | true
 * false    | ||        | true      | true
 * false    | ||        | false     | false
 * -----------------------------------------
 *
 * ! (Oposite of the value it self)
 * -----------------
 * value1   | result
 * -----------------
 * !true    | fasle
 * !false   | true
 * -----------------
 */

fn main(){
    let and_operator: bool = true && false;
    println!("and_operator = {}", and_operator);

    let or_operator: bool = true || false;
    println!("or_operator = {}", or_operator);

    println!("negation = {}", !false);
}
