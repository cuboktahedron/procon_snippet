#[allow(dead_code)]
mod matrix {
  use cargo_snippet::snippet;

  #[snippet]
  fn matrix2_mul(a: &Vec<Vec<usize>>, b: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let mut c = vec![vec![0; b[0].len()]; a.len()];
    for i in 0..a.len() {
      for j in 0..b.len() {
        for k in 0..b[0].len() {
          c[i][k] += a[i][j] * b[j][k];
        }
      }
    }

    c
  }

  #[snippet]
  fn matrix2_mul_mod(a: &Vec<Vec<usize>>, b: &Vec<Vec<usize>>, md: usize) -> Vec<Vec<usize>> {
    let mut c = vec![vec![0; b[0].len()]; a.len()];
    for i in 0..a.len() {
      for j in 0..b.len() {
        for k in 0..b[0].len() {
          c[i][k] = (c[i][k] + a[i][j] * b[j][k]) % md;
        }
      }
    }

    c
  }

  #[snippet]
  #[snippet(include = "matrix2_mul_mod")]
  fn matrix2_pow_mod(a: &Vec<Vec<usize>>, k: usize, md: usize) -> Vec<Vec<usize>> {
    let mut a = a.clone();

    let mut m = k;
    let mut ret = {
      let len = a.len();
      let mut unit = vec![vec![0usize; len]; len];
      for i in 0..len {
        unit[i][i] = 1;
      }

      unit
    };

    while m > 0 {
      if m & 1 == 1 {
        ret = matrix2_mul_mod(&ret, &a, md);
      }

      a = matrix2_mul_mod(&a, &a, md);
      m = m / 2;
    }

    ret
  }

  #[cfg(test)]
  mod tests {
    use super::*;
    #[test]
    fn test_matrix_mul() {
      let a = vec![vec![1, 2], vec![3, 4]];
      let b = vec![vec![5, 6], vec![7, 8]];
      let c = vec![vec![19, 22], vec![43, 50]];
      assert_eq!(matrix2_mul(&a, &b), c);

      let a = vec![vec![1, 2]];
      let b = vec![vec![3], vec![4]];
      let c = vec![vec![11]];
      assert_eq!(matrix2_mul(&a, &b), c);

      let a = vec![vec![2]];
      let b = vec![vec![3]];
      let c = vec![vec![6]];
      assert_eq!(matrix2_mul(&a, &b), c);
    }

    #[test]
    fn test_matrix_pow_mod() {
      let a = vec![vec![1, 2], vec![3, 4]];
      let b = vec![vec![37, 54], vec![20, 57]];
      assert_eq!(matrix2_pow_mod(&a, 3, 61), b);
    }
  }
}
