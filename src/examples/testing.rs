fn sum(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod sum_tests {
    use super::*;

    #[test]
    fn sum_should_produce_correct_result() {
        let s = sum(2, 2);
        assert_eq!(s, 4);
        assert_ne!(s, 5);
        assert!(s == 4);
    }
}

#[cfg(test)]
mod panic_tests {
    #[test]
    #[should_panic]
    fn greater_than_100() {
        let v = vec![1, 2, 3];
        v[100];
    }

    #[test]
    #[should_panic(expected = "Some Message")]
    fn panics_with_correct_message() {
        panic!("Some Message");
    }
}
