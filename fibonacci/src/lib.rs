pub fn fib(n: u32) -> u64 {
    match n {
        0 => panic!("O is invalid argument to fibonacci"),
        1 | 2 => 1,
        _ => fib(n-1) + fib(n-2)
    }
}

pub struct Fib {
  index: u32,
  value: u64,
  prev: u64
}

impl Fib {
  pub fn new(start: Option<u32>) -> Fib {
    let index = start.unwrap_or(0);
    let value = fib(index);
    let prev = fib(index - 1);
    Fib { index, value, prev }
  }
}

impl Iterator for Fib {
  type Item = u64;

  fn next(&mut self) -> Option<Self::Item> {
      self.index += 1;
      let prev = self.prev;
      self.prev = self.value;
      self.value += prev;

      if self.index <= 10 {
          Some(self.value)
      } else {
          None
      }
  }
}



#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let mut fib7 = Fib::new(Some(7));
        assert_eq!(13, fib7.value);
        assert_eq!(Some(21), fib7.next());
        assert_eq!(Some(34), fib7.next());
        assert_eq!(Some(55), fib7.next());
        assert_eq!(None, fib7.next());
    }
}
