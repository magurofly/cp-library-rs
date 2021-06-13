# グラフライブラリ

<details>
  <summary>コード</summary>

```rust
pub mod graph {
  #![allow(unused_imports, dead_code)]

  pub use vec_graph::VecGraph;
  pub use sub_graph::SubGraph;
  
  pub trait Graph<V, E> {
    type Vertex: Vertex<V>;
    type Edge: Edge<E>;
    
    fn vertex(&self, id: usize) -> &Self::Vertex;
    fn edge(&self, id: usize) -> &Self::Edge;
    
    /// Number of vertices
    fn n(&self) -> usize;
    
    /// Number of edges
    fn m(&self) -> usize;
    
    fn edges_from(&self, from: usize) -> Vec<usize>;
    fn adjacent_vertices(&self, from: usize) -> Vec<usize>;

    fn each_edge_from<F: FnMut(&Self::Edge)>(&self, from: usize, mut f: F) {
      for e in self.edges_from(from) {
        (f)(self.edge(e));
      }
    }

    fn each_adjacent_vertex<F: FnMut(&Self::Vertex)>(&self, from: usize, mut f: F) {
      for v in self.adjacent_vertices(from) {
        (f)(self.vertex(v));
      }
    }

    fn find_edge(&self, from: usize, to: usize) -> Option<usize> {
      self.edges_from(from).into_iter().filter(|&e| self.edge(e).to() == to ).next()
    }

    fn reverse_edge(&self, e: usize) -> Option<usize> {
      let edge = self.edge(e);
      self.find_edge(edge.to(), edge.from())
    }
    
    fn walk<F: FnMut(&mut Walker, usize)>(&self, from: usize, mut f: F) {
      let mut walker = Walker::new(self.n());
      walker.go_next(from);
      while let Some(u) = walker.next() { (f)(&mut walker, u) }
    }
    
    /// do DFS
    /// edge is passed to `f`
    /// Note that `from` vertex is not passed to `f`
    fn dfs<F: FnMut(&Self::Edge)>(&self, from: usize, mut f: F) {
      self.walk(from, |walker, u| self.each_edge_from(u, |edge| if walker.go_next(edge.to()) { (f)(edge) } ) );
    }
    
    /// do BFS
    /// edge is passed to `f`
    /// Note that `from` vertex is not passed to `f`
    fn bfs<F: FnMut(&Self::Edge)>(&self, from: usize, mut f: F) {
      self.walk(from, |walker, u| self.each_edge_from(u, |edge| if walker.go_later(edge.to()) { (f)(edge) } ) );
    }
    
    fn shortest_path_bfs<T: Measure>(&self, from: usize) -> Vec<Option<T>> {
      let mut dist = vec![None; self.n()];
      dist[from] = Some(T::zero());
      self.bfs(from, |edge| dist[edge.to()] = dist[edge.from()].map(|d| d + T::one() ) );
      dist
    }

    fn shortest_path_bfs_by<C: Measure, F: FnMut(&Self::Edge, C) -> Option<C>>(&self, from: usize, mut f: F) -> Vec<Option<C>> {
      let mut dist = vec![None; self.n()];
      dist[from] = Some(C::zero());
      self.walk(from, |walker, u| self.each_edge_from(u, |edge| {
        if let Some(d) = (f)(edge, dist[u].unwrap()) {
          if walker.go_later(edge.to()) {
            dist[edge.to()] = Some(d);
          }
        }
      }));
      dist
    }

    /// `f(e, d)? >= d`
    fn shortest_path_dijkstra_by<C: Measure, F: FnMut(&Self::Edge, C) -> Option<C>>(&self, from: usize, mut f: F) -> Vec<Option<C>> {
      let mut dist = vec![None; self.n()];
      let mut pq = BinaryHeap::new();
      dist[from] = Some(C::zero());
      pq.push((Reverse(C::zero()), from));
      while let Some((Reverse(d1), u)) = pq.pop() {
        if dist[u] != Some(d1) { continue }
        self.each_edge_from(u, |edge| if let Some(d) = (f)(edge, d1) {
          dist[edge.to()].if_chmin(d, || pq.push((Reverse(d), edge.to())) );
        } );
      }
      dist
    }

    fn shortest_path_dijkstra(&self, from: usize) -> Vec<Option<E>> where E: Measure {
      self.shortest_path_dijkstra_by(from, |edge, d| Some(d + *edge.weight()) )
    }

    fn shortest_path_spfa_by<C: Measure, F: FnMut(&Self::Edge, C) -> Option<C>>(&self, from: usize, mut f: F) -> Vec<Option<C>> {
      let mut dist = vec![None; self.n()];
      dist[from] = Some(C::zero());
      let mut q = Uniqueue::new();
      q.push_front(from);
      while let Some(u) = q.pop_back() {
        self.each_edge_from(u, |edge| {
          let v = edge.to();
          if let Some(d) = (f)(edge, dist[u].unwrap()) {
            dist[v].if_chmin(d, || q.push_front(v) );
          }
        });
      }
      dist
    }

    fn shortest_path_spfa(&self, from: usize) -> Vec<Option<E>> where E: Measure {
      self.shortest_path_spfa_by(from, |edge, d| Some(d + *edge.weight()) )
    }

    // fn shortest_paths_floyd_warshall(&self, loops: bool) -> Vec<Vec<Option<C>>> {
    //   let mut dist = vec![vec![None; self.n()]; self.n()];
    //   for i in 0 .. self.n() {

    //   }
    // }

    fn minimum_spanning_tree_prim_by<C: Measure, F: FnMut(&Self::Edge) -> Option<C>>(&self, root: usize, mut f: F) -> (C, SubGraph<'_, V, E, Self>) {
      let mut cost = C::zero();
      let mut included = vec![false; self.n()];
      let mut vertices = vec![root];
      let mut edges = Vec::new();
      let mut pq = self.edges_from(root).into_iter().filter_map(|e| (f)(self.edge(e)).map(|c| (Reverse(c), e) ) ).collect::<BinaryHeap<_>>();
      included[root] = true;
      while let Some((Reverse(c), e)) = pq.pop() {
        let to = self.edge(e).to();
        if included[to] { continue }
        included[to] = true;
        vertices.push(to);
        edges.push(e);
        cost += c;
        for x in self.edges_from(to).into_iter().filter_map(|e| (f)(self.edge(e)).map(|c| (Reverse(c), e) ) ) {
          pq.push(x);
        }
      }
      (cost, SubGraph::new(self, vertices, edges))
    }

    fn minimum_spanning_tree_prim(&self, root: usize) -> (E, SubGraph<'_, V, E, Self>) where E: Measure {
      self.minimum_spanning_tree_prim_by(root, |edge| Some(*edge.weight()) )
    }
  }
  
  pub trait GraphMut<V, E>: Graph<V, E> where Self::Vertex: VertexMut<V>, Self::Edge: EdgeMut<E> {
    fn add_vertex(&mut self, weight: V) -> usize;
    fn add_vertices<I: IntoIterator<Item = V>>(&mut self, weights: I) -> Vec<usize> {
      weights.into_iter().map(|weight| self.add_vertex(weight) ).collect::<Vec<_>>()
    }
    fn add_default_vertices(&mut self, n: usize) -> Vec<usize> where V: Default {
      (0 .. n).map(|_| self.add_vertex(Default::default()) ).collect::<Vec<_>>()
    }
    fn add_arc(&mut self, from: usize, to: usize, weight: E) -> usize;
    fn connect(&mut self, from: usize, to: usize) -> usize where E: Default {
      self.add_arc(from, to, Default::default())
    }
    fn add_edge(&mut self, u: usize, v: usize, weight: E) -> (usize, usize) where E: Clone {
      (self.add_arc(u, v, weight.clone()), self.add_arc(v, u, weight))
    }
    fn connect2(&mut self, u: usize, v: usize) -> (usize, usize) where E: Default {
      (self.connect(u, v), self.connect(v, u))
    }

    fn add_arc_residual(&mut self, from: usize, to: usize, weight: E) -> (usize, usize) where E: Measure {
      assert!(from < self.n() && to < self.n());
      (self.add_arc(from, to, weight), self.add_arc(to, from, E::zero()))
    }

    fn add_arcs<I: IntoIterator>(&mut self, edges: I) -> Vec<usize> where I::Item: IntoEdge<E> {
      edges.into_iter().map(|edge| {
        let (from, to, weight) = edge.into_edge();
        self.add_arc(from, to, weight)
      }).collect::<Vec<_>>()
    }
    fn add_edges<I: IntoIterator>(&mut self, edges: I) -> Vec<(usize, usize)> where E: Clone, I::Item: IntoEdge<E> {
      edges.into_iter().map(|edge| {
        let (from, to, weight) = edge.into_edge();
        self.add_edge(from, to, weight)
      }).collect::<Vec<_>>()
    }
    
    fn vertex_mut(&mut self, id: usize) -> &mut Self::Vertex;
    fn edge_mut(&mut self, id: usize) -> &mut Self::Edge;
    
    fn each_edge_mut_from<F: FnMut(&mut Self::Edge)>(&mut self, from: usize, f: F);
    fn each_adjacent_vertex_mut<F: FnMut(&mut Self::Vertex)>(&mut self, from: usize, f: F);
  }
  
  pub trait Vertex<V> {
    fn weight(&self) -> &V;
  }
  
  pub trait Edge<E> {
    fn from(&self) -> usize;
    fn to(&self) -> usize;
    fn weight(&self) -> &E;
  }

  pub trait VertexMut<V>: Vertex<V> {
    fn weight_mut(&mut self) -> &mut V;
  }

  pub trait EdgeMut<E>: Edge<E> {
    fn weight_mut(&mut self) -> &mut E;
  }

  pub trait IntoEdge<E> {
    fn into_edge(self) -> (usize, usize, E);
  }
  
  #[derive(Debug)]
  pub struct Walker {
    visited: Vec<bool>,
    queue: VecDeque<usize>,
  }
  
  pub mod vec_graph {
    use super::{Graph, GraphMut, measure::Measure};
    
    #[derive(Debug)]
    pub struct Vertex<V> {
      weight: V,
      edges: Vec<usize>,
    }
    impl<V> super::Vertex<V> for Vertex<V> {
      fn weight(&self) -> &V { &self.weight }
    }
    impl<V> super::VertexMut<V> for Vertex<V> {
      fn weight_mut(&mut self) -> &mut V { &mut self.weight }
    }
    
    #[derive(Debug)]
    pub struct Edge<E> {
      from: usize,
      to: usize,
      weight: E,
      rev: Option<usize>,
    }
    impl<E> super::Edge<E> for Edge<E> {
      fn from(&self) -> usize { self.from }
      fn to(&self) -> usize { self.to }
      fn weight(&self) -> &E { &self.weight }
    }
    impl<E> super::EdgeMut<E> for Edge<E> {
      fn weight_mut(&mut self) -> &mut E { &mut self.weight }
    }
    
    #[derive(Debug)]
    pub struct VecGraph<V, E> {
      vertices: Vec<Vertex<V>>,
      edges: Vec<Edge<E>>,
    }
    impl<V, E> VecGraph<V, E> {
      pub fn new() -> Self {
        Self {
          vertices: Vec::new(),
          edges: Vec::new(),
        }
      }
    }
    impl<V, E> Graph<V, E> for VecGraph<V, E> {
      type Vertex = Vertex<V>;
      type Edge = Edge<E>;
      
      fn n(&self) -> usize { self.vertices.len() }
      fn m(&self) -> usize { self.edges.len() }
      
      fn vertex(&self, id: usize) -> &Vertex<V> {
        assert!(id < self.n());
        &self.vertices[id]
      }

      fn edge(&self, id: usize) -> &Edge<E> {
        assert!(id < self.m());
        &self.edges[id]
      }
      
      fn edges_from(&self, from: usize) -> Vec<usize> {
        assert!(from < self.n());
        self.vertices[from].edges.clone()
      }
      
      fn each_edge_from<F: FnMut(&Self::Edge)>(&self, from: usize, mut f: F) {
        assert!(from < self.n());
        for &e in &self.vertices[from].edges {
          (f)(&self.edges[e]);
        }
      }
      
      fn adjacent_vertices(&self, from: usize) -> Vec<usize> {
        assert!(from < self.n());
        self.vertices[from].edges.iter().map(|&e| self.edges[e].to ).collect::<Vec<_>>()
      }
      
      fn each_adjacent_vertex<F: FnMut(&Self::Vertex)>(&self, from: usize, mut f: F) {
        assert!(from < self.n());
        for &e in &self.vertices[from].edges {
          (f)(&self.vertices[self.edges[e].to]);
        }
      }

      fn reverse_edge(&self, e: usize) -> Option<usize> {
        self.edge(e).rev
      }
    }
    impl<V, E> GraphMut<V, E> for VecGraph<V, E> {
      fn add_vertex(&mut self, weight: V) -> usize {
        let id = self.n();
        self.vertices.push(Vertex { weight, edges: Vec::new() });
        id
      }
      fn add_arc(&mut self, from: usize, to: usize, weight: E) -> usize {
        assert!(from < self.n() && to < self.n());
        let id = self.m();
        self.edges.push(Edge { from, to, weight, rev: None });
        self.vertices[from].edges.push(id);
        id
      }

      fn add_edge(&mut self, from: usize, to: usize, weight: E) -> (usize, usize) where E: Clone {
        assert!(from < self.n() && to < self.n());
        let id = self.m();
        self.edges.push(Edge { from, to, weight: weight.clone(), rev: Some(id + 1) });
        self.edges.push(Edge { from: to, to: from, weight, rev: Some(id) });
        self.vertices[from].edges.push(id);
        self.vertices[to].edges.push(id + 1);
        (id, id + 1)
      }
      
      fn vertex_mut(&mut self, id: usize) -> &mut Self::Vertex {
        assert!(id < self.n());
        &mut self.vertices[id]
      }
      fn edge_mut(&mut self, id: usize) -> &mut Self::Edge {
        assert!(id < self.m());
        &mut self.edges[id]
      }
      
      fn each_edge_mut_from<F: FnMut(&mut Self::Edge)>(&mut self, from: usize, mut f: F) {
        assert!(from < self.n());
        for &e in &self.vertices[from].edges {
          (f)(&mut self.edges[e]);
        }
      }
      fn each_adjacent_vertex_mut<F: FnMut(&mut Self::Vertex)>(&mut self, from: usize, mut f: F) {
        assert!(from < self.n());
        for v in self.adjacent_vertices(from) {
          (f)(&mut self.vertices[v]);
        }
      }
    }
  }

  pub mod sub_graph {
    use super::*;
    use std::marker::PhantomData;

    #[derive(Debug)]
    pub struct SubGraph<'a, V, E, G: Graph<V, E> + ?Sized> {
      origin: &'a G,
      vertices: Vec<usize>,
      edges: Vec<usize>,
      phantom: PhantomData<(V, E)>,
    }
    impl<'a, V, E, G: Graph<V, E> + ?Sized> SubGraph<'a, V, E, G> {
      pub fn new(origin: &'a G, vertices: Vec<usize>, edges: Vec<usize>) -> Self {
        Self { origin, vertices, edges, phantom: PhantomData }
      }
    }
    impl<V, E, G: Graph<V, E> + ?Sized> Graph<V, E> for SubGraph<'_, V, E, G> {
      type Vertex = G::Vertex;
      type Edge = G::Edge;

      fn n(&self) -> usize { self.vertices.len() }
      fn m(&self) -> usize { self.edges.len() }

      fn vertex(&self, id: usize) -> &Self::Vertex {
        assert!(id < self.n());
        self.origin.vertex(self.vertices[id])
      }

      fn edge(&self, id: usize) -> &Self::Edge {
        assert!(id < self.m());
        self.origin.edge(self.edges[id])
      }

      fn edges_from(&self, from: usize) -> Vec<usize> {
        assert!(from < self.n());
        self.origin.edges_from(self.vertices[from]).into_iter().filter_map(|e| self.edges.binary_search(&e).ok() ).collect::<Vec<_>>()
      }

      fn adjacent_vertices(&self, from: usize) -> Vec<usize> {
        assert!(from < self.n());
        self.origin.adjacent_vertices(self.vertices[from]).into_iter().filter_map(|v| self.vertices.binary_search(&v).ok() ).collect::<Vec<_>>()
      }
    }
  }
  
  impl Walker {
    fn new(n: usize) -> Self {
      Self {
        visited: vec![false; n],
        queue: VecDeque::new(),
      }
    }
    
    fn go_next(&mut self, v: usize) -> bool {
      assert!(v < self.visited.len());
      if self.visited[v] { return false; }
      self.visited[v] = true;
      self.queue.push_back(v);
      true
    }
    
    fn go_later(&mut self, v: usize) -> bool {
      assert!(v < self.visited.len());
      if self.visited[v] { return false; }
      self.visited[v] = true;
      self.queue.push_front(v);
      true
    }
    
    fn forget(&mut self, v: usize) {
      assert!(v < self.visited.len());
      self.visited[v] = false;
    }
  }
  impl Iterator for Walker {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
      self.queue.pop_back()
    }
  }

  impl<E> IntoEdge<E> for (usize, usize, E) {
    fn into_edge(self) -> (usize, usize, E) {
      self
    }
  }
  impl<E: Default> IntoEdge<E> for (usize, usize) {
    fn into_edge(self) -> (usize, usize, E) {
      (self.0, self.1, Default::default())
    }
  }
  
  pub mod measure {
    use std::cmp::*;
    use num_traits::*;
    
    pub trait AssignOps: Sized + std::ops::AddAssign + std::ops::SubAssign + std::ops::MulAssign + std::ops::DivAssign + std::ops::RemAssign {}
    pub trait Measure: std::fmt::Debug + Num + Default + Ord + Copy + AssignOps + std::iter::Sum {
      fn chmin(&mut self, other: Self) -> &mut Self {
        if *self > other {
          *self = other;
        }
        self
      }
      fn chmax(&mut self, other: Self) -> &mut Self {
        if *self < other {
          *self = other;
        }
        self
      }
      fn if_chmin<F: FnOnce()>(&mut self, other: Self, procedure: F) -> &mut Self {
        if *self > other {
          *self = other;
          (procedure)();
        }
        self
      }
      fn if_chmax<F: FnOnce()>(&mut self, other: Self, procedure: F) -> &mut Self {
        if *self < other {
          *self = other;
          (procedure)();
        }
        self
      }
    }
    pub trait MeasureSigned: Measure + Signed {}
    
    pub trait OptionUtil<T>: Sized {
      fn unwrap(self) -> T;
      fn is_some(&self) -> bool;
      fn insert(&mut self, value: T) -> &mut T;
      fn chmin(&mut self, other: T) -> &mut Self where Self: Clone, T: Clone + Ord {
        let value = if self.is_some() { self.clone().unwrap().min(other) } else { other };
        self.insert(value);
        self
      }
      fn chmax(&mut self, other: T) -> &mut Self where Self: Clone, T: Clone + Ord {
        let value = if self.is_some() { self.clone().unwrap().max(other) } else { other };
        self.insert(value);
        self
      }
      fn and_if<F: FnOnce(T) -> bool>(self, predicate: F) -> bool {
        self.is_some() && predicate(self.unwrap())
      }
      fn if_chmin<F: FnOnce()>(&mut self, other: T, procedure: F) -> &mut Self where Self: Clone, T: Clone + Ord {
        if !self.is_some() || self.clone().unwrap() > other {
          self.insert(other);
          procedure();
        }
        self
      }
      fn if_chmax<F: FnOnce()>(&mut self, other: T, procedure: F) -> &mut Self where Self: Clone, T: Clone + Ord {
        if !self.is_some() || self.clone().unwrap() < other {
          self.insert(other);
          procedure();
        }
        self
      }
    }
    impl<T> OptionUtil<T> for Option<T> {
      fn unwrap(self) -> T {
        Option::<T>::unwrap(self)
      }
      fn is_some(&self) -> bool {
        Option::<T>::is_some(self)
      }
      fn insert(&mut self, value: T) -> &mut T {
        *self = Some(value);
        self.as_mut().unwrap()
      }
    }
    
    impl<T: Sized + std::ops::AddAssign + std::ops::SubAssign + std::ops::MulAssign + std::ops::DivAssign + std::ops::RemAssign> AssignOps for T {}
    impl<T: std::fmt::Debug + Copy + Ord + Default + Num + AssignOps + std::iter::Sum> Measure for T {}
    impl<T: Signed + Measure> MeasureSigned for T {}
    
    
    fn zip<T, U>(left: Option<T>, right: Option<U>) -> Option<(T, U)> {
      left.and_then(|x| right.map(|y| (x, y) ))
    }
  }

  struct Uniqueue<T> {
    queue: VecDeque<T>,
    inq: IHashSet<T>,
  }
  impl<T: Clone + Eq + std::hash::Hash> Uniqueue<T> {
    fn new() -> Self {
      Self { queue: VecDeque::new(), inq: FxHashSet::default() }
    }

    fn push_front(&mut self, value: T) {
      if self.inq.contains(&value) { return };
      self.inq.insert(value.clone());
      self.queue.push_front(value);
    }

    fn pop_back(&mut self) -> Option<T> {
      if let Some(value) = self.queue.pop_back() {
        self.inq.remove(&value);
        Some(value)
      } else {
        None
      }
    }
  }

  type IHashSet<T> = HashSet<T, core::hash::BuildHasherDefault<FxHasher>>;
  
  use measure::*;
  use std::collections::*;
  use std::cmp::*;
  use itertools::*;
  use num_traits::*;
  use rustc_hash::*;
}
```
  
</details>
