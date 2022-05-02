#[allow(dead_code)]
mod a {
  use cargo_snippet::snippet;

  #[snippet(name = "inf_2_60u")]
  const INF: usize = 2 << 60;
}

#[allow(dead_code)]
mod b {
  use cargo_snippet::snippet;

  #[snippet(name = "inf_10_18u")]
  const INF: usize = 100_0000_0000_0000_0000;
}

#[allow(dead_code)]
mod c {
  use cargo_snippet::snippet;

  #[snippet(name = "inf_2_60i")]
  const INF: isize = 2 << 60;
}

#[allow(dead_code)]
mod d {
  use cargo_snippet::snippet;

  #[snippet(name = "inf_10_18i")]
  const INF: isize = 100_0000_0000_0000_0000;
}
