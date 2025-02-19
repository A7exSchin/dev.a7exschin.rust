fn main() {
    let x: i32 = 5;
    let y: i32 = x+4;

    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    let z: i32 = add(x, y);

    println!("The value of x+y is: {}", z);


}

fn add(x: i32, y: i32) -> i32 {
    let z: i32 = x + y;
    // This is a return expression :o
    z
}
