pub fn add(a: i64, b: i64) -> i64 {
    a + b
}

pub fn sub(a: i64, b: i64) -> i64 {
    a - b
}

pub fn mul(a: i64, b: i64) -> i64 {
    a * b
}

pub fn div(a: i64, b: i64) -> Option<i64> {
    if b == 0 {
        None
    } else {
        Some(a / b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_works() {
        assert_eq!(add(2, 3), 5);
        assert_eq!(add(-1, 1), 0);
    }

    #[test]
    fn sub_works() {
        assert_eq!(sub(5, 3), 2);
        assert_eq!(sub(0, 4), -4);
    }

    #[test]
    fn mul_works() {
        assert_eq!(mul(4, 5), 20);
        assert_eq!(mul(-2, 3), -6);
    }

    #[test]
    fn div_works() {
        assert_eq!(div(10, 2), Some(5));
        assert_eq!(div(7, 0), None);
    }
}
