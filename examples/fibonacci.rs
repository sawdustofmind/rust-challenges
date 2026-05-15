/// Computes the nth Fibonacci number using an iterative approach.
fn fibonacci(n: u32) -> u64 {
    if n <= 2 {
        return if n > 0 { 1 } else { 0 };
    }

    let mut prev = 1;
    let mut curr = 1;
    for _ in 2..n {
        (prev, curr) = (curr, prev + curr);
    }
    curr
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibonacci() {
        assert_eq!(fibonacci(1), 1);
        assert_eq!(fibonacci(2), 1);
        assert_eq!(fibonacci(3), 2);
        assert_eq!(fibonacci(4), 3);
        assert_eq!(fibonacci(10), 55);
        assert_eq!(fibonacci(50), 12586269025);
    }
}

