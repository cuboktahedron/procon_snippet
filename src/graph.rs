#[allow(dead_code)]
mod graph {
  use cargo_snippet::snippet;
  use std::collections::BinaryHeap;

  #[snippet(doc_hidden)]
  // 始点から最短経路を求める。
  // 辺の重みが非負数の場合に使用できる。
  // 計算量: O(N^2) N: 頂点数
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
  // 計算量: 疎グラフの場合：O(|N|log|N|)
  //         密グラフの場合：Θ(|N^2|log|N|) N: 頂点数
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
  // 計算量: O(N^3) N: 頂点数
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

  #[snippet(doc_hidden)]
  // 鏡連結成分分解(SCC: Storongly Connected Component)
  // 計算量: O(|N + E|) : N: 頂点数、E: 辺の数
  fn scc(n: usize, g: &Vec<Vec<usize>>) -> Vec<usize> {
    // DFSで末端の頂点から順に通し番号を振る
    fn dfs(v: usize, g: &Vec<Vec<usize>>, used: &mut Vec<bool>, vs: &mut Vec<usize>) {
      used[v] = true;
      for &i in &g[v] {
        if !used[i] {
          dfs(i, g, used, vs);
        }
      }
      vs.push(v);
    }

    let mut used = vec![false; n];
    let mut vs = vec![];
    for i in 0..n {
      if !used[i] {
        dfs(i, g, &mut used, &mut vs);
      }
    }

    // グラフの向きを反転したうえで
    // 上で振った通し番号の大きい方の頂点から再度DFSして到達できる頂点を
    // 同じグループとしてマークする

    fn rdfs(v: usize, rg: &Vec<Vec<usize>>, used: &mut Vec<bool>, cmp: &mut Vec<usize>, k: usize) {
      used[v] = true;
      cmp[v] = k;
      for &i in &rg[v] {
        if !used[i] {
          rdfs(i, rg, used, cmp, k);
        }
      }
    }

    let mut rg = vec![vec![]; n];
    for i in 0..n {
      for &j in &g[i] {
        rg[j].push(i)
      }
    }

    let mut used = vec![false; n];
    let mut k = 0;
    let mut cmp = vec![0usize; n];
    for i in (0..vs.len()).rev() {
      if !used[vs[i]] {
        rdfs(vs[i], &rg, &mut used, &mut cmp, k);
        k += 1;
      }
    }

    cmp
  }
}
