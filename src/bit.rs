#[allow(dead_code)]
mod bit {
  use cargo_snippet::snippet;

  #[snippet]
  fn enumerate_bits(n: usize, m: usize) -> Vec<bool> {
    let mut v = vec![false; m];

    for i in 0..m {
      v[i] = (n >> i) & 1 == 1;
    }

    v
  }

  #[cfg(test)]
  mod tests {
    use super::*;

    #[test]
    fn test_enumerate_bits() {
      assert_eq!(enumerate_bits(15, 4), vec![true, true, true, true]);
      assert_eq!(enumerate_bits(11, 4), vec![true, true, false, true]);
    }
  }
}
