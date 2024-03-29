#[allow(dead_code)]
mod probability {
  use crate::integer::interger::pow_mod;
  use cargo_snippet::snippet;

  #[snippet]
  fn combination(n: usize, r: usize) -> usize {
    let mut ans = 1usize;
    for i in 1..=r {
      ans = ans * (n - i + 1) / i;
    }

    ans
  }

  #[snippet(include = "pow_mod")]
  fn combination_mod(n: usize, r: usize, md: usize) -> usize {
    let mut factorials = vec![0usize; n + 1];
    factorials[0] = 1;
    for i in 1..=n {
      factorials[i] = (i * factorials[i - 1]) % md;
    }

    let fact_n = factorials[n];
    let fact_r = factorials[r];
    let fact_nr = factorials[n - r];

    let fact_r1 = pow_mod(fact_r, md - 2, md);
    let fact_nr1 = pow_mod(fact_nr, md - 2, md);

    (((fact_n * fact_r1) % md) * fact_nr1) % md
  }

  #[snippet(include = "pow_mod")]
  fn combination_mod2(n: usize, m: usize, md: usize) -> usize {
    let m = m.min(n - m);
    let mut a = 1usize;
    let mut b = 1usize;

    for i in 0..m {
      a = (a * (n - i)) % md;
      b = (b * (i + 1)) % md;
    }

    return (a * pow_mod(b, md - 2, md)) % md;
  }

  #[snippet(include = "combination_mod")]
  fn combination_dup_mod(n: usize, k: usize, md: usize) -> usize {
    combination_mod(n + k - 1, k, md)
  }

  #[cfg(test)]
  mod tests {
    use super::*;

    #[test]
    fn test_combination() {
      assert_eq!(combination(1, 1), 1);
      assert_eq!(combination(3, 3), 1);
      assert_eq!(combination(4, 2), 6);
      assert_eq!(combination(4, 0), 1);
    }

    #[test]
    fn test_combination_mod() {
      assert_eq!(combination_mod(1, 1, 31), 1);
      assert_eq!(combination_mod(8, 3, 31), 25);
    }

    #[test]
    fn test_combination_mod2() {
      assert_eq!(combination_mod2(1, 1, 31), 1);
      assert_eq!(combination_mod2(8, 5, 31), 25);
    }

    #[test]
    fn test_combination_dup_mod() {
      assert_eq!(combination_dup_mod(1, 1, 31), 1);
      assert_eq!(combination_dup_mod(6, 4, 31), 2);
    }
  }
}
