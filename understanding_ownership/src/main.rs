fn main() {
    {
        let mut s = String::from("Hello, world!");
        println!("{}", s);
        s.push_str(" Wie gets?");
        println!("{}", s);
    }
    //println!("{}", s); won't compile, s is out of scope here
    
    let s1 = String::from("hello");
    let s2 = s1;
    // println!("{}", s1); won't compile, s1 is no longer valid
    println!("{}", s2);

    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);

    //Ownership with functions
    let s = String::from("hello");
    takes_ownership(s);
   // println!("{}", s);  won't compile, s is no longer valid

   let x = 5;
   makes_copy(x);
   println!("{}", x); // x is still valid because i32 implements the Copy trait

    let s1 = gives_ownership();
    println!("{}", s1);

    let s2 = String::from("hello again");
    let s3 = takes_and_gives_back(s2);
   // println!("{}", s2);
    println!("{}", s3);
    let t5 = calculate_length(&s3);
    println!("The length of '{}' is {}.", s3, t5);

    //Mutable references
    let mut s = String::from("hello");
    change(&mut s);
    println!(" mutable s is {}", s);

    //Slice Types
    let mut s = String::from("hello world");
    let word_index = first_word(&s);
    println!("The first word ends at index {}", word_index);
    s.clear(); // this empties the String, making it equal to ""
    println!("The string after clear is '{}'", s);
    println!("The first word ends at index {}", word_index);

    //String Slices
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("{} {}", hello, world);

    let s1 = String::from("hello world");
    let first_word = first_word_slice(&s1);
    println!("The first word is: {}", first_word);

    //Other Slice Types
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    println!("The slice is: {:?}", slice);  
    assert_eq!(slice, &[2, 3]);

}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello ownership giver");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn first_word(s: &String) -> usize{
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word_slice(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}   