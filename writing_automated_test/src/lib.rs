pub fn add<T>(left: T, right: T) -> T
where
    T: std::ops::Add<Output = T>,
{
    left + right
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
}
