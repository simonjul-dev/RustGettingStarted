#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}





fn main() {
    let mut v: Vec<i32> = Vec::new();
   // v =  vec![1, 2, 3];
    println!("{:?}", v);
    v.push(1);
    v.push(2);
    v.push(3);
    v.push(4);
    println!("{:?}", v);

    let _third = &v[2];
    /*println!("The third element is {}", third);
    let ninth = v.get(9);
    match ninth {
        Some(ninth) => println!("The ninth element is {}", ninth),
        None => println!("There is no ninth element."),
    }*/
    v.push(5);
    
    //Iterating over values
    for i in &v {
        println!("{}", i);
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
    for i in &v {
        println!("{}", i);
    }

    //Using Enums to Store Multiple Types
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    println!("{:?}", row);

    let a: i32;

    {
        let v = vec![1, 2, 3];
        a = v[0];
    }
    println!("{}", a);

    //String
    let s1 = String::from("hello");
    let s2 = String::from(", world");
    let s3 = s1 + &s2; // note s1 has been moved
    println!("{}", s3);

    //HashMap
    use std::collections::HashMap;
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    let team_name = String::from("Bluze");
    let score = scores.get(&team_name);
    match score {
        Some(score) => println!("Score: {}", score),
        None => println!("No score found"),
    }

    //Updating a Value Based on the Old Value
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);


}
