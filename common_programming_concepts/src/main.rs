use std::io;
fn main() {
    
    // Mutable and immutable variables
    let mut x = 5;
    println!("The value of x is {x}");
    x = 6;
    println!("The value of x is {x}");
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Three hours in seconds is {THREE_HOURS_IN_SECONDS}");

    let y : i32 = 5;
    println!("The value of y is {y}");

    let y: u32 = 6;
    println!("The value of y is {y}");

    let space = "   ";
    let space = space.len();
    println!("The length of space is {space}");

    let _guess : u32 = "42".parse().expect("Not a number!");

    let _guess : u32 = match "42".parse(){
        Ok(num) => num,
        Err(_) => {
            println!("Not a number!");
            0
        }
    };

    // Floating-point types
    let decimal = 98_222.5;
    let hex = 0xff;
    let octal = 0o77;
    let binary = 0b1111_0000;
    let byte = b'A';
    println!("decimal: {decimal}, hex: {hex}, octal: {octal}, binary: {binary}, byte: {byte}");

    // Arithmetic operations    
    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 4.0 / 30.1;
    let remainder = 43 % 5;
    let truncated = -5 / 3;
    println!("sum: {sum}, difference: {difference}, product: {product}, quotient: {quotient}, remainder: {remainder}, truncated: {truncated}");   

    // Boolean type
    let t = true;
    let f: bool = false;
    println!("t: {t}, f: {f}");

    // Character type
    let c = 'z';
    let z: char = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("c: {c}, z: {z}, heart_eyed_cat: {heart_eyed_cat}");

    // Tuple type
    let tuple = (500, 6.4, 1);
    let (x, y, z) = tuple;
    println!("x: {x}, y: {y}, z: {z}");
    let x = tuple;
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
    println!("five_hundred: {five_hundred}, six_point_four: {six_point_four}, one: {one}");


    // Array type
    let array = [1, 2, 3, 4, 5];
    let first = array[0];
    let second = array[1];
    println!("first: {first}, second: {second}");

    //Invalid array index handling
    let _a = [1, 2, 3, 4, 5];

    /*println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
    */
    let x = 10;
    let y = 20;

    // Function call
    another_function(x, y);

    //Statement and expressions
    let x = {
        let y = 6;
        y + 1
    };
    println!("The value of x is: {x}");

    let returned_five = five();
    println!("The value returned from five() is: {returned_five}");
    let plus_one_result = plus_one(5);
    println!("The value returned from plus_one(5) is: {plus_one_result}");

    //Control flow
    let number = 6;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

        //Loop labels
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10; 
        
        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    }

    for element in a {
        println!("the value is: {element}");
    }
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");

}

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {x}");
    println!("The value of y is: {y}");
}

//Function with return value
fn five() -> i32 {
    5
}

fn plus_one(a: i32) -> i32 {
    a + 1
}