fn main() {
    // integers 8/16/32/64/128 bits possible

    let x: i8 = 7;
    println!("{}", x);

    // signed (>=0) or unsigned

    let y: u8 = 10; //always positive
    println!("{}", y);

    // integer literals: 255 in different systems
    let decimal = 02_55;
    let hex = 0xff;
    let octal = 0o377;
    let binary = 0b1111_1111;

    println!("Dec: {}; Hex: {}; Octal: {}; Binary: {}", decimal, hex, octal, binary);

    let byte = b'A';
    println!("Bytes: {}", byte);

    // _ in fornt of the var silences the warnigns of an unused variable
    let _silence_warnings = 5;


    // floats
    let _z = 2.0; //f64 as default bc on moderns cpus almost same speed as f32, bettter precision
    let _h: f32 = 1.0;

    //booleans, true,false are keywords (type inferred)
    let _t = true;

    //specifically set type
    let _f: bool = false;

    //Character type
    let c = 'c';

    println!("Char: {}", c);

    //calculations

    let a = 10;
    let b = 4;

    let remainder = a % b;

    println!("Remainder: {}", remainder);


    // Tuples

    let tup = (500, "hi", true);

    println!("Tuples value at first position: {}", tup.0);
    println!("Tuples value at second position: {}", tup.1);
    println!("Tuples value at third position: {}", tup.2);

    //mega cool: direkte variable zuweisen

    let (q, w, e) = tup;
    println!("q: {}; w: {}; e: {}", q, w, e);
}
