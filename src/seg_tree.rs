use cargo_snippet::snippet;

#[allow(dead_code)]
#[snippet(doc_hidden)]
mod lazy_segtree {
  // ---------- begin Lazy Segment Tree ----------
  pub trait LazySegmentTreeNode {
    type Value: Clone;
    type Lazy: Clone;

    /// 下位ノードの確定値から自身のノードの確定値を計算する
    fn fold(l: &Self::Value, r: &Self::Value) -> Self::Value;

    /// 確定値と遅延部から確定値を計算する
    fn eval(x: &Self::Value, f: &Self::Lazy) -> Self::Value;

    /// 遅延部と遅延部から次の遅延部を計算する
    fn merge(g: &Self::Lazy, h: &Self::Lazy) -> Self::Lazy;

    /// 単位元
    /// 確定値の初期値であり値が更新されていない場合は、結果に影響を与えない値を使う
    fn e() -> Self::Value;

    /// 恒等関数
    /// 遅延部の初期値
    fn id() -> Self::Lazy;
  }

  pub struct LazySegmentTree<Node: LazySegmentTreeNode> {
    size: usize,
    bit: usize,
    nodes: Vec<(Node::Value, Node::Lazy)>,
  }

  impl<Node: LazySegmentTreeNode> LazySegmentTree<Node> {
    pub fn new(n: usize) -> LazySegmentTree<Node> {
      let size = n.next_power_of_two();
      let bit = size.trailing_zeros() as usize;
      LazySegmentTree {
        size: size,
        bit: bit,
        nodes: vec![(Node::e(), Node::id()); 2 * size],
      }
    }

    pub fn build_by(z: &[Node::Value]) -> LazySegmentTree<Node> {
      let mut seg = LazySegmentTree::<Node>::new(z.len());
      for (node, z) in seg.nodes[seg.size..].iter_mut().zip(z.iter()) {
        node.0 = z.clone();
      }
      let nodes = &mut seg.nodes;
      for i in (1..seg.size).rev() {
        nodes[i].0 = Node::fold(&nodes[2 * i].0, &nodes[2 * i + 1].0);
      }
      seg
    }

    fn apply(&mut self, x: usize, op: &Node::Lazy) {
      let node = &mut self.nodes[x];

      node.0 = Node::eval(&node.0, op);
      node.1 = Node::merge(&node.1, op);
    }

    /// 指定したノードの遅延部を下位のノードに伝播する
    fn propagate(&mut self, x: usize) {
      let mut op = Node::id();

      // 自身のノードの遅延部をクリアしつつ伝播する値を取得
      std::mem::swap(&mut op, &mut self.nodes[x].1);

      // 子供のノードに取得した遅延部を適用する
      self.apply(2 * x, &op);
      self.apply(2 * x + 1, &op);
    }

    /// 指定した範囲の遅延部を下位ノードに伝播する
    fn propagate_range(&mut self, l: usize, r: usize) {
      // 末端のノードのインデックスを計算
      let x = l + self.size;
      let y = r + self.size;

      // 範囲に関係するノードを上から順に見ていって伝播する
      let mut k = self.bit;
      while (x >> k) == (y >> k) {
        self.propagate(x >> k);
        k -= 1;
      }

      for i in ((x.trailing_zeros() as usize + 1)..=k).rev() {
        self.propagate(x >> i);
      }

      for i in ((y.trailing_zeros() as usize + 1)..=k).rev() {
        self.propagate(y >> i);
      }
    }

    fn save_range(&mut self, l: usize, r: usize) {
      // 末端のノードのインデックスを計算
      let mut x = l + self.size;
      let mut y = r + self.size;

      // 範囲に完全に含まれていない、より下にあるノードから確定した値を使って
      //　上位のノードの値を確定していく
      let mut p = (x & 1) == 1;
      let mut q = (y & 1) == 1;
      x >>= 1;
      y >>= 1;
      while 0 < x && x < y {
        if p {
          self.nodes[x].0 = Node::fold(&self.nodes[2 * x].0, &self.nodes[2 * x + 1].0);
        }
        if q {
          self.nodes[y].0 = Node::fold(&self.nodes[2 * y].0, &self.nodes[2 * y + 1].0);
        }
        p |= (x & 1) == 1;
        q |= (y & 1) == 1;
        x >>= 1;
        y >>= 1;
      }
      while 0 < x {
        self.nodes[x].0 = Node::fold(&self.nodes[2 * x].0, &self.nodes[2 * x + 1].0);
        x >>= 1;
      }
    }

    pub fn update(&mut self, l: usize, r: usize, op: Node::Lazy) {
      // 関係する範囲の遅延部を下位ノードに伝播する
      self.propagate_range(l, r);

      // 末端のノードのインデックスを計算
      let mut x = l + self.size;
      let mut y = r + self.size;

      // 末端のノードから共通の先祖にぶつかるまで
      while x < y {
        if x & 1 == 1 {
          self.apply(x, &op);
          x += 1;
        }
        if y & 1 == 1 {
          y -= 1;
          self.apply(y, &op);
        }
        x >>= 1;
        y >>= 1;
      }

      self.save_range(l, r);
    }

    pub fn find(&mut self, l: usize, r: usize) -> Node::Value {
      // 関係する範囲の遅延部を下位ノードに伝播する
      self.propagate_range(l, r);

      // 末端のノードのインデックスを計算
      let mut x = l + self.size;
      let mut y = r + self.size;

      let mut p = Node::e();
      let mut q = Node::e();
      while x < y {
        if x & 1 == 1 {
          p = Node::fold(&p, &self.nodes[x].0);
          x += 1;
        }
        if y & 1 == 1 {
          y -= 1;
          q = Node::fold(&self.nodes[y].0, &q);
        }
        x >>= 1;
        y >>= 1;
      }
      Node::fold(&p, &q)
    }
  }
  // ---------- end Lazy Segment Tree ----------
}

use lazy_segtree::LazySegmentTreeNode;

#[snippet("lazy_segtree_node")]
struct Node;

#[snippet("lazy_segtree_node_impl")]
#[snippet(include = "lazy_segtree_node")]
#[snippet(include = "lazy_segtree")]
#[snippet(prefix = "use lazy_segtree::LazySegmentTree;")]
#[snippet(prefix = "use lazy_segtree::LazySegmentTreeNode;")]
impl LazySegmentTreeNode for Node {
  type Value = usize;
  type Lazy = usize;

  fn fold(l: &Self::Value, r: &Self::Value) -> Self::Value {
    std::cmp::max(*l, *r)
  }

  fn eval(x: &Self::Value, f: &Self::Lazy) -> Self::Value {
    std::cmp::max(*x, *f)
  }

  fn merge(g: &Self::Lazy, h: &Self::Lazy) -> Self::Lazy {
    std::cmp::max(*g, *h)
  }

  fn e() -> Self::Value {
    0
  }

  fn id() -> Self::Lazy {
    0
  }
}
