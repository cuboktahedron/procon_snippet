#[allow(dead_code)]
mod trigonometric {
  use cargo_snippet::snippet;
  use core::f64::consts::PI;

  #[snippet(prefix = "use core::f64::consts::PI;")]
  fn rad_to_deg(rad: f64) -> f64 {
    rad / PI * 180f64
  }

  #[snippet(prefix = "use core::f64::consts::PI;")]
  fn deg_to_rad(deg: f64) -> f64 {
    deg * PI / 180f64
  }

  #[cfg(test)]
  mod tests {
    use super::*;

    #[test]
    fn test_deg_to_rad() {
      assert_eq!(deg_to_rad(0f64), 0f64);
      assert_eq!(deg_to_rad(90f64), PI / 2f64);
      assert_eq!(deg_to_rad(180f64), PI);
      assert_eq!(deg_to_rad(270f64), PI * 3f64 / 2f64);
      assert_eq!(deg_to_rad(360f64), 2f64 * PI);
    }

    #[test]
    fn test_rad_to_deg() {
      assert_eq!(rad_to_deg(0f64), 0f64);
      assert_eq!(rad_to_deg(PI / 2f64), 90f64);
      assert_eq!(rad_to_deg(PI), 180f64);
      assert_eq!(rad_to_deg(PI * 3f64 / 2f64), 270f64);
      assert_eq!(rad_to_deg(2f64 * PI), 360f64);
    }
  }
}
