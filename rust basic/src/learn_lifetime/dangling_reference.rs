fn main() {
    let x: &u8;
    {
        let y: u8 = 11;
        x = &y;
    }
    println!("x = {}", x);
}
