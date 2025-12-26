pub fn add<T>(left: T, right: T) -> T
where
    T: std::ops::Add<Output = T>,
{
    left + right
}

fn greeting(name: &str) -> String {
    format!("Hello, {}!", name)
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn it_works2() {
        let result = add(2.0, 2.0);
        assert_eq!(result, 4.000000000);
    }

    #[test]
    fn greeting_contains_name() {
        let name = "Carol";
        let result = greeting(name);
        assert!(result.contains(name));
    }


    #[test]    #[should_panic(expected = "between 1 and 100")]
    fn greater_than_100() {
        Guess::new(200);
    }

    #[test]
    #[ignore]
    fn it_works3() -> Result<(), String> {
        let result = add(2, 2);

        if result == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}
