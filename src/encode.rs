#[allow(dead_code)]
mod encode {
  use cargo_snippet::snippet;

  #[snippet]
  fn run_length(s: impl Into<String>) -> Vec<(char, usize)> {
    let mut v = vec![];

    let s = s.into();
    if s.len() == 0 {
      return v;
    }

    let mut prev = s.chars().next().unwrap();
    let mut seq = 0;
    for c in s.chars() {
      if c == prev {
        seq += 1;
      } else {
        v.push((prev, seq));
        seq = 1;
        prev = c;
      }
    }

    v.push((prev, seq));

    v
  }

  #[cfg(test)]
  mod tests {
    use super::*;

    #[test]
    fn test_run_length() {
      assert_eq!(run_length("aaabcccc"), vec![('a', 3), ('b', 1), ('c', 4)]);
      assert_eq!(run_length(""), vec![]);
      assert_eq!(run_length(String::from("ab")), vec![('a', 1), ('b', 1)]);
    }
  }
}
