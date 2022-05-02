#[allow(dead_code)]
mod graph {
  use cargo_snippet::snippet;

  #[snippet(doc_hidden)]
  // 全頂点間の最短経路を求める。
  // 経路がない箇所には[INF]を設定しておくこと。
  // 計算量: O(n^3)
  fn warshall_floyd(mut g: Vec<Vec<usize>>, n: usize) {
    for k in 0..n {
      for i in 0..n {
        for j in 0..n {
          g[i][j] = g[i][j].min(g[i][k] + g[k][j]);
        }
      }
    }
  }
}
