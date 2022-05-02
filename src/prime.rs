#[allow(dead_code)]
mod prime {
  use cargo_snippet::snippet;

  #[snippet]
  #[snippet(include = "primes_or_not")]
  fn enumerate_primes(n: usize) -> Vec<usize> {
    let v = primes_or_not(n)
      .iter()
      .enumerate()
      .filter(|(_, &b)| b)
      .map(|(n, _)| n)
      .collect::<Vec<usize>>();

    v
  }

  #[snippet]
  fn primes_or_not(n: usize) -> Vec<bool> {
    let mut pn = vec![true; n + 1];
    pn[0] = false;
    pn[1] = false;
    let mut i = 2;
    while i * i <= n {
      if pn[i] {
        for j in (i + i..=n).step_by(i) {
          pn[j] = false;
        }
      }

      i += 1;
    }

    pn
  }

  #[cfg(test)]
  mod tests {
    use super::*;

    #[test]
    fn test_enumerate_primes() {
      assert_eq!(enumerate_primes(10), vec![2, 3, 5, 7]);
    }
    #[test]
    fn test_primes_or_not() {
      assert_eq!(
        primes_or_not(6),
        vec![false, false, true, true, false, true, false]
      );
    }
  }
}
