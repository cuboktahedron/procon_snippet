#[allow(dead_code)]
mod graph {
  use cargo_snippet::snippet;
  use std::collections::BinaryHeap;

  #[snippet(doc_hidden)]
  // 始点から最短経路を求める。
  // 辺の重みが非負数の場合に使用できる。
  // 計算量: O(n^2)
  fn dijkstra(g: &Vec<Vec<(usize, usize)>>, st: usize) -> Vec<usize> {
    const INF: usize = 100_0000_0000_0000_0000;

    let v_num = g.len();
    let mut dist = vec![INF; v_num];
    let mut used = vec![false; v_num];

    dist[st] = 0;

    // 次の緩和元となる一番コストが小さい頂点を探す
    for _ in 0..v_num {
      let mut min_dist = INF;
      let mut min_j = INF;
      for j in 0..v_num {
        if !used[j] && dist[j] < min_dist {
          min_dist = dist[j];
          min_j = j;
        }
      }

      if min_j == INF {
        // 始点から到達できる頂点がなくなったので終了
        break;
      }

      for &(u, w) in &g[min_j] {
        dist[u] = dist[u].min(min_dist + w);
      }

      used[min_j] = true;
    }

    dist
  }

  #[snippet(doc_hidden)]
  // 始点から最短経路を求める。
  // 計算量: 疎グラフの場合：O(|n|log|n|)
  //         密グラフの場合：Θ(|n^2|log|n|)
  fn dijkstra2(g: &Vec<Vec<(usize, isize)>>, st: usize) -> Vec<isize> {
    const INF: isize = 100_0000_0000_0000_0000;

    let v_num = g.len();
    let mut dist = vec![INF; v_num];
    dist[st] = 0;

    let mut hepa = BinaryHeap::new();
    hepa.push((0isize, st));

    while let Some((d, v)) = hepa.pop() {
      let d = -d;

      if d > dist[v] {
        // 既により小さいコストでルートが見つかっているためため処理しない
        continue;
      }

      for &(u, w) in &g[v] {
        let d = d + w;
        if d < dist[u] {
          // コストが小さくなるので緩和する
          dist[u] = d;

          // min-heapにしたいため正負を反対させている
          hepa.push((-d, u));
        }
      }
    }

    dist
  }

  #[snippet(doc_hidden)]
  // 全頂点間の最短経路を求める。
  // 経路がない箇所には[INF]を設定しておくこと。
  // 計算量: O(n^3)
  fn warshall_floyd(g: &mut Vec<Vec<usize>>) {
    let v_num = g.len();

    for k in 0..v_num {
      for i in 0..v_num {
        for j in 0..v_num {
          g[i][j] = g[i][j].min(g[i][k] + g[k][j]);
        }
      }
    }
  }
}
