

fn main() {
    println!("Hello, world!");
}

fn add_two(x: i32) -> i32 {
    x + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_two_positive() {
        assert_eq!(add_two(3), 5);
    }

    #[test]
    fn test_add_two_negative() {
        assert_eq!(add_two(-3), -1);
    }

    #[test]
    fn test_add_two_zero() {
        assert_eq!(add_two(0), 2);
    }
}
