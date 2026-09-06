/* Ownership
 *
 * Ownership is special rust feature, this feature used for enheancing memory management
 *
 * Rules:
 * - Every values in rust must have owner (variable)
 * - in one time only allowed one owner
 * - When owner out of the scope, the value will be vanish
 *
 */

fn main(){

    // Owenership Rules
    // we all know, most of the programming language will exectue their code sequentialy
    // From top to bottom, means we cannot access a value if it is not declare

    // println!("x = {}", x); <----- we can not do this
    let x = 10;
    println!("x = {}", x);

    {
        let y = 5;
        println!("x = {}", x);
        println!("y = {}", y);

    }

    // we cannot do this, because the 'y' already dead,
    // its only lived in curly branches or scope function

    // println!("y = {}", y);
    println!("x = {}", x);

}

#[test]
fn copy_data(){
    /* As mention erlyer, owenership only have one owner at one time
     * if we declare a variabel that reference to other variabel
     * and its variabel is fixed, or it stored in STACK MEMORY
     * it will just create a new memory in STACK MEMORY
     * and it will copy the data from it reference
     */ 

    let p: u8 = 20;
    let q: u8 = p;

    // the flow goes like this:
    // p set 20
    // q set whatever p have value i want to copy it
    // and i will make my own memory
    // both have same value but they are have different meomory allocation

    println!("p = {}, q = {}", p, q);
    assert_eq!(p, q);
}

#[test]
fn ownership_movement(){
    /* reference to other variabel when its located in HEAP MEMORY
     * it has different behavior, its not means copy. its mean change
     * as mention erlyer, ownership only have one owner at one time
     * means if we declare variable that reference to other variabel
     * and its variabel is located in HEAP MEMORY, it will not only copy the data
     * it also will change the owner
     */

    let name = String::from("Marc Marquez");
    println!("his name is {}", name);

    let rider = name;

    // println!("his name is {}", name); <-- we cannot do this, because the variable already dont
    // have any owner

    println!("The rider is {}", rider);

}

#[test]
fn clone(){
    /* What if i realy realy want to copy data from variable that stored in HEAP MEMORY
     * Fortunatly rust already have a solution for you.
     * the solution is .clone() function, what is this function for?
     * this function will copy the data and it will create a new MEMORY allocation in HEAP
     * They have same data but different owner or location in HEAP MEMORY
     */

    let mango = String::from("Mango");
    let manga = mango.clone();

    assert_eq!(mango, manga);

    println!("Mango = {}, Manga = {}", mango, manga);
}
