struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

  struct Color(i32, i32, i32);
  struct Point(i32, i32, i32);
struct AlwaysEqual;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}   

fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        sign_in_count: 1,
        active: true,
    };
    print_user(&user1);
    user1.email = String::from("another@example.com");
    print_user(&user1);
    let user2 = build_user(String::from("another@example.com"), String::from("anotherusername456"));
    print_user(&user2);
    let user3 = User {
        email: String::from("user3@example.com"),
        ..user2
    };
    print_user(&user3);
    //print_user(&user2); won't compile because user2 was moved to user3

    //Creating a tuple struct
    let _black = Color(0, 0, 0);
    let _origin = Point(0, 0, 0);

    //Defining Unit-like struct
    let _subject = AlwaysEqual;

    //Adding Functionality with Derived Traits
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("\nrect1 is {rect1:#?} ");

    //Using dbg! to Print Debug Information
    let scale = 2;
    let rect2 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };
    dbg!(&rect2);

    //Method Syntax
    let rect3 = Rectangle {
        width: 40,
        height: 60,
    };
    println!("\nThe area of the rectangle is {} square pixels.", rect3.area());
    let area = rect3.area();
    println!("\nThe area of the rectangle is {area} square pixels.");

    //Using Methods to Compare Structs
    let rect4 = Rectangle {
        width: 30,
        height: 40,
    };
    let rect5 = Rectangle {
        width: 20,
        height: 30,
    };
    println!(
        "\nCan rect4 hold rect5? {}",
        rect4.can_hold(&rect5)
    );


}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        sign_in_count: 1,
        active: true,
    }
}

fn print_user(user: &User) {
    println!("\nUsername: {}", user.username);
    println!("Email: {}", user.email);
    println!("Sign-in count: {}", user.sign_in_count);
    println!("Active: {}", user.active);
}