#[allow(dead_code)]
mod interger {
  use cargo_snippet::snippet;

  #[snippet]
  fn sum_digits(n: usize) -> usize {
    let mut n = n;
    let mut sum = 0usize;
    while n > 0 {
      let q = n % 10;
      sum += q;
      n /= 10;
    }

    sum
  }

  #[snippet]
  fn sum_digits_of_chars(n: &Vec<char>) -> usize {
    let mut sum = 0usize;
    for c in n {
      let q = (*c as u8 - b'0') as usize;
      sum += q;
    }

    sum
  }

  #[snippet]
  fn sum_1_to_n(n: usize) -> usize {
    (n + 1) * n / 2
  }

  #[snippet]
  #[snippet(include = "sum_1_to_n")]
  fn sum_m_to_n(m: usize, n: usize) -> usize {
    assert!(m <= n);

    if m == 0 {
      sum_1_to_n(n)
    } else {
      sum_1_to_n(n) - sum_1_to_n(m - 1)
    }
  }

  #[cfg(test)]
  mod tests {
    use super::*;

    #[test]
    fn test_sum_digits() {
      assert_eq!(sum_digits(0), 0);
      assert_eq!(sum_digits(1234567890), 45);
    }

    #[test]
    fn test_sum_digits_of_chars() {
      assert_eq!(sum_digits_of_chars(&vec!['0']), 0);

      let digits = 1234567890.to_string().chars().collect::<Vec<char>>();
      assert_eq!(sum_digits_of_chars(&digits), 45);
    }

    #[test]
    fn test_sum_1_to_n() {
      assert_eq!(sum_1_to_n(0), 0);
      assert_eq!(sum_1_to_n(1), 1);
      assert_eq!(sum_1_to_n(10), 55);
    }

    #[test]
    fn test_sum_m_to_n() {
      assert_eq!(sum_m_to_n(0, 0), 0);
      assert_eq!(sum_m_to_n(0, 1), 1);
      assert_eq!(sum_m_to_n(5, 10), 45);
    }
  }
}
