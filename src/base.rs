#[allow(dead_code)]
mod base {
  use cargo_snippet::snippet;

  #[snippet]
  fn base_n_to_decimal(src: &Vec<char>, base: usize) -> usize {
    assert!(base > 1);

    let mut v10 = 0usize;
    for i in 0..src.len() {
      v10 = v10 * base + (src[i] as u8 - b'0') as usize;
    }

    v10
  }

  #[snippet]
  fn decimal_to_base_n(src: usize, base: usize) -> Vec<char> {
    assert!(base > 1);

    if src == 0 {
      return vec!['0'];
    }

    let mut v = vec![];
    let mut n = src;
    while n > 0 {
      let q = n % base;
      v.push((q as u8 + b'0') as char);
      n /= base;
    }

    v.into_iter().rev().collect::<Vec<_>>()
  }

  #[snippet]
  #[snippet(include = "base_n_to_decimal")]
  #[snippet(include = "decimal_to_base_n")]
  fn base_n_to_m(src: &Vec<char>, n: usize, m: usize) -> Vec<char> {
    assert!(n > 1);
    assert!(m > 1);

    let vn = base_n_to_decimal(src, n);
    decimal_to_base_n(vn, m)
  }

  #[cfg(test)]
  mod tests {
    use super::*;

    fn n2vc(n: usize) -> Vec<char> {
      n.to_string().chars().collect::<Vec<char>>()
    }

    #[test]
    fn test_base_n_to_usize() {
      assert_eq!(base_n_to_decimal(&n2vc(110), 2), 6);
      assert_eq!(base_n_to_decimal(&n2vc(21), 8), 17);
    }

    #[test]
    fn test_usize_to_base_n() {
      assert_eq!(decimal_to_base_n(6, 2), n2vc(110));
      assert_eq!(decimal_to_base_n(17, 8), n2vc(21));
    }

    #[test]
    fn test_base_n_to_m() {
      assert_eq!(base_n_to_m(&n2vc(13), 8, 9), n2vc(12));
      assert_eq!(base_n_to_m(&n2vc(12), 9, 8), n2vc(13));
    }
  }
}
