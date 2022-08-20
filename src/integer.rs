#[allow(dead_code)]
pub mod interger {
  use cargo_snippet::snippet;
  use std::collections::HashMap;

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

  #[snippet(doc_hidden)]
  // 繰り返し二乗法
  // n^m % md
  // 計算量: O(logN)
  pub fn pow_mod(n: usize, m: usize, md: usize) -> usize {
    let mut m = m;
    let mut n = n;

    let mut ret = 1;

    while m > 0 {
      if m & 1 == 1 {
        ret = (ret * n) % md;
      }

      n = n * n % md;
      m = m / 2;
    }

    ret
  }

  #[snippet(doc_hidden)]
  // 素因数分解
  // 計算量: O(√N)
  fn factorize_primes(n: usize) -> HashMap<usize, usize> {
    let mut n = n;
    let mut map = HashMap::new();

    let mut i = 2;
    while i * i <= n {
      while n % i == 0 {
        n = n / i;
        *map.entry(i).or_insert(0) += 1;
      }

      i += 1;
    }

    if n > 1 {
      map.insert(n, 1);
    }

    map
  }

  #[snippet(doc_hidden)]
  // 素因数分解
  // 計算量: O(√N)
  fn factorize_primes_flatten(n: usize) -> Vec<usize> {
    let mut n = n;
    let mut v = vec![];

    let mut i = 2;
    while i * i <= n {
      while n % i == 0 {
        n = n / i;
        v.push(i);
      }

      i += 1;
    }

    if n > 1 {
      v.push(n);
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

    #[test]
    fn test_pow_mod() {
      assert_eq!(pow_mod(0, 0, 100), 1);
      assert_eq!(pow_mod(0, 1, 100), 0);
      assert_eq!(pow_mod(1, 0, 100), 1);
      assert_eq!(pow_mod(5, 4, 2000), 625);
      assert_eq!(pow_mod(4, 5, 2000), 1024);
      assert_eq!(pow_mod(4, 5, 1000), 24);
    }

    #[test]
    fn test_factorize_primes() {
      assert_eq!(factorize_primes(0), HashMap::new());
      assert_eq!(factorize_primes(1), HashMap::new());

      let mut map = HashMap::new();
      map.insert(2, 1);
      map.insert(3, 1);
      map.insert(7, 1);
      assert_eq!(factorize_primes(42), map);

      let mut map = HashMap::new();
      map.insert(2, 4);
      map.insert(3, 1);
      assert_eq!(factorize_primes(48), map);
    }

    #[test]
    fn test_factorize_primes_flatten() {
      assert_eq!(factorize_primes_flatten(0), vec![]);
      assert_eq!(factorize_primes_flatten(1), vec![]);
      assert_eq!(factorize_primes_flatten(42), vec![2, 3, 7]);
      assert_eq!(factorize_primes_flatten(48), vec![2, 2, 2, 2, 3]);
    }
  }
}
