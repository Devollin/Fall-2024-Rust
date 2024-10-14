fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn multiply(a: i32, b: i32) -> i32 {
    let mut total = 0;

    for _ in 0..b {
        total = add(total, a);
    }
    
    total
}

fn raise(base: i32, power: i32) -> i32 {
    let mut total = base;

    for _ in 1..power {
        total = multiply(total, base)
    }

    total
}

fn main() {
    println!("2 + 2 = {}", add(2, 2));
    println!("3 * 4 = {}", multiply(3, 4));
    println!("3 ^ 3 = {}", raise(3, 3));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 2), 4);
    }

    #[test]
    fn test_multiply() {
        assert_eq!(multiply(2, 4), 8);
        assert_eq!(multiply(3, 4), 12);
    }

    #[test]
    fn test_raise() {
        assert_eq!(raise(2, 4), 16);
        assert_eq!(raise(3, 3), 27);

        assert!(f32::abs(0.3333 - (raise(3, -1) as i32)) < 0.005);
    }

    #[test]
    fn test_add_multiple() {
        let test_cases = vec![
            (1, 1, 2),
            (0, 0, 0),
            (-1, 1, 0),
            (100, -50, 50)
        ];

        for (a, b, expected) in test_cases {
            assert_eq!(add(a, b), expected, "Failed on input ({}, {})", a, b);
        }
    }
}