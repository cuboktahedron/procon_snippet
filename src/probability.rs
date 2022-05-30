#[allow(dead_code)]
mod probability {
  use cargo_snippet::snippet;

  #[snippet]
  fn combination(n: usize, r: usize) -> usize {
    let mut ans = 1usize;
    for i in 1..=r {
      ans = ans * (n - i + 1) / i;
    }

    ans
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
  }
}
