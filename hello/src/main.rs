fn main() {
    let mut x = 5;
    println!("The value of x is {}", x);


    // mutable variable is let mut (veränderbar)
    x = 6;

    println!("the value of x is {}", x)

    // lives as long as the program is alive: use for envvars? - all caps
    // constants always immutable
    const SECONDS: i8 = 7;

    println!("Value of seconds {}", SECONDS)
}
