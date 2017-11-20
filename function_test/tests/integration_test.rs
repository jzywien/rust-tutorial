extern crate function_test;

use function_test::fibonacci;

#[test]
fn test_fibonacci() {
  let result = fibonacci::fib(8);
  assert_eq!(result, 21);
}