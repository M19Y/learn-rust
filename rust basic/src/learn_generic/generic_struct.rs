struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let integer: Point<u8> = Point::<u8> { x: 9, y: 11 };
    println!("Integer = X: {}, Y:{}", integer.x, integer.y);

    let floating: Point<f64> = Point::<f64> { x: 9.11, y: 11.01 };
    println!("Floating = X: {}, Y:{}", floating.x, floating.y);
}
