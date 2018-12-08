pub mod calculator {
    pub fn add_two(a: i32) -> i32 {
        super::internal_adder(a, 2)
    }
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    #[should_panic(expected = "Make this test fail!")]
    fn another_test_must_fail() {
        panic!("Make this test fail!");
    }

    use super::*;

    #[test]
    fn internal() {
        let actual = internal_adder(2, 2);
        assert_eq!(4, actual);
    }

    use super::calculator;
    
    #[test]
    fn module_test() {
        let result = calculator::add_two(1);
        assert_eq!(3, result);
    }
}
