pub fn fib(n: u32) -> u64 {
    match n {
        0 => panic!("O is invalid argument to fibonacci"),
        1 | 2 => 1,
        _ => fib(n-1) + fib(n-2)
    }
}

#[cfg(test)]
mod tests {
    use fibonacci;

    #[test]
    fn test_fib() {
      let result = fibonacci::fib(8);
      assert_eq!(result, 21);
    }

    #[test]
    #[should_panic]
    fn test_invalid() {
      fibonacci::fib(0);
    }
}
