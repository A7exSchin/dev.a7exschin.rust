fn main() {
    let x = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }
    println!("The value of x is: {}", x);

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("The value of the tuple is: {:?}", tup);
    println!("The value of the first tuple entry is: {}", tup.0);
    println!("The value of the second tuple entry is: {}", tup.1);
    println!("The value of the third tuple entry is: {}", tup.2);

    let (x, y, z) = tup;
    println!("The value of x is: {x}, y is: {y}, z is: {z}");

    let a: [i32; 2] = [1, 2];
    println!("The value of the array is: {:?}", a);
    println!("The value of the array in pretty is: {:#?}", a);
    println!("The value of the first array entry is: {}", a[0]);
    println!("The value of the second array entry is: {}", a[1]);

    println!("The length of the array is: {}", a.len());

    another_function();
  }

  fn another_function() {
    println!("Another function.");
  }
