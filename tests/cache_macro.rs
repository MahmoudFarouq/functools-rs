use functools::cache;
use std::collections::HashMap;

#[cache]
fn fibonacci(m: usize, n: usize) -> (usize, usize) {
    if n < 2 {
        (n, 0)
    } else {
        let t1 = fibonacci(m, n - 1);
        let t2 = fibonacci(m, n - 2);

        (t1.0 + t2.0, t1.1 + t2.1)
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn test_fibonacci() {
        assert_eq!(fibonacci(0, 1), (1, 0));
        assert_eq!(fibonacci(0, 8), (21, 0));
        assert_eq!(fibonacci(0, 9), (34, 0));
        assert_eq!(fibonacci(0, 10), (55, 0));
        assert_eq!(fibonacci(0, 50), (12586269025, 0));
    }
}
