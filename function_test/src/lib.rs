pub mod number;
pub mod fibonacci;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn get_suffix_tests() {
    assert_eq!(number::get_suffix_for_num(1), "st");
    assert_eq!(number::get_suffix_for_num(2), "nd");
    assert_eq!(number::get_suffix_for_num(3), "rd");
    assert_eq!(number::get_suffix_for_num(4), "th");
    assert_eq!(number::get_suffix_for_num(5), "th");
    assert_eq!(number::get_suffix_for_num(10), "th");
    assert_eq!(number::get_suffix_for_num(11), "th");
    assert_eq!(number::get_suffix_for_num(12), "th");
    assert_eq!(number::get_suffix_for_num(13), "th");
    assert_eq!(number::get_suffix_for_num(21), "st");
    assert_eq!(number::get_suffix_for_num(22), "nd");
  }
}