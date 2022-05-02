#[allow(dead_code)]
mod interger {
  use cargo_snippet::snippet;

  #[snippet]
  fn abs_diff(lhs: usize, rhs: usize) -> usize {
    if lhs < rhs {
      rhs - lhs
    } else {
      lhs - rhs
    }
  }

  #[snippet]
  #[snippet(include = "gcd")]
  fn lcm(a: usize, b: usize) -> usize {
    a / gcd(a, b) * b
  }

  #[snippet]
  fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
      a
    } else {
      gcd(b, a % b)
    }
  }

  #[snippet]
  fn enumerate_divs(n: usize) -> Vec<usize> {
    let mut v = vec![];

    let mut i = 1;
    while i * i <= n {
      if n % i == 0 {
        v.push(i);
        if i * i != n {
          v.push(n / i);
        }
      }

      i += 1;
    }

    v
  }

  #[cfg(test)]
  mod tests {
    use super::*;

    #[test]
    fn test_abs_diff() {
      assert_eq!(abs_diff(8, 6), 2);
      assert_eq!(abs_diff(6, 8), 2);
      assert_eq!(abs_diff(8, 8), 0);
    }

    #[test]
    fn test_lcm() {
      assert_eq!(lcm(6, 8), 24);
    }

    #[test]
    fn test_gcd() {
      assert_eq!(lcm(6, 8), 24);
    }

    #[test]
    fn test_enum_divs() {
      assert_eq!(enumerate_divs(7), [1, 7]);
      assert_eq!(enumerate_divs(10), [1, 10, 2, 5]);
      assert_eq!(enumerate_divs(32), [1, 32, 2, 16, 4, 8]);
    }
  }
}
