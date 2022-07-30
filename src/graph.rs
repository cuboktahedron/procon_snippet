#[allow(dead_code)]
mod graph {
  use cargo_snippet::snippet;

  #[snippet(doc_hidden)]
  // 始点から最短経路を求める。
  // 経路がない箇所には[INF]を設定しておくこと。
  // 辺の重みが非負数の場合に使用できる。
  // 計算量: O(n^2)
  fn dijkstra(g: &Vec<Vec<usize>>, s: usize) -> Vec<usize> {
    const INF: usize = 100_0000_0000_0000_0000;

    let n = g.len();
    let mut dist = vec![INF; n];
    let mut used = vec![false; n];

    dist[s] = 0;

    for _ in 0..n {
      let mut min_dist = INF;
      let mut min_j = INF;
      for j in 0..n {
        if !used[j] && dist[j] < min_dist {
          min_dist = dist[j];
          min_j = j;
        }
      }

      if min_j == INF {
        break;
      }

      for j in 0..n {
        dist[j] = dist[j].min(dist[min_j] + g[min_j][j]);
      }

      used[min_j] = true;
    }

    dist
  }

  #[snippet(doc_hidden)]
  // 全頂点間の最短経路を求める。
  // 経路がない箇所には[INF]を設定しておくこと。
  // 計算量: O(n^3)
  fn warshall_floyd(g: &mut Vec<Vec<usize>>, n: usize) {
    for k in 0..n {
      for i in 0..n {
        for j in 0..n {
          g[i][j] = g[i][j].min(g[i][k] + g[k][j]);
        }
      }
    }
  }
}
