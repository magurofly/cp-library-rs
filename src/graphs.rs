pub mod graphs {
  // Last Update: 2021-06-17 20:20
  #![allow(unused_imports, dead_code)]
  
  pub use dic_graph::{DicGraph, VecGraph, HashGraph};
  pub use sub_graph::SubGraph;
  
  pub trait Graph<E = ()> {
    type Vertex: Vertex;
    type Edge: Edge<E>;
    
    fn vertex(&self, id: usize) -> &Self::Vertex;
    fn edge(&self, id: usize) -> &Self::Edge;
    
    /// Number of vertices
    fn n(&self) -> usize;
    
    /// Number of edges
    fn m(&self) -> usize;
    
    fn edges_from(&self, from: usize) -> Vec<usize>;
    fn adjacent_vertices(&self, from: usize) -> Vec<usize>;

    fn each_edge_from(&self, from: usize, mut f: impl FnMut(usize)) {
      for e in self.edges_from(from) { (f)(e) }
    }

    fn each_adjacent_vertex(&self, from: usize, mut f: impl FnMut(usize)) {
      for v in self.adjacent_vertices(from) { (f)(v) }
    }

    fn find_edge(&self, from: usize, to: usize) -> Option<usize> {
      self.edges_from(from).into_iter().filter(|&e| self.edge(e).to() == to ).next()
    }

    fn reverse_edge(&self, e: usize) -> Option<usize> {
      let edge = self.edge(e);
      self.find_edge(edge.to(), edge.from())
    }
    
    fn walk(&self, from: usize, mut f: impl FnMut(&mut Walker, usize)) {
      let mut walker = Walker::new();
      walker.go_next(from);
      while let Some(u) = walker.next() { (f)(&mut walker, u) }
    }
    
    /// do DFS in preorder
    /// edge is passed to `f`
    /// Note that `from` vertex is not passed to `f`
    fn dfs(&self, from: usize, mut f: impl FnMut(&Self::Edge)) {
      self.walk(from, |walker, u| self.each_edge_from(u, |e| if walker.go_next(self.edge(e).to()) { (f)(self.edge(e)) } ) );
    }

    fn eulertour(&self, from: usize, mut f: impl FnMut(&Self::Edge)) {
      let mut visited = vec![false; self.m()];
      let mut stack = self.edges_from(from);
      for &e in &stack { visited[e] = true }
      while let Some(e) = stack.pop() {
        (f)(self.edge(e));
        self.each_edge_from(self.edge(e).to(), |d| {
          if !visited[d] {
            visited[d] = true;
            stack.push(d);
          }
        });
      }
    }
    
    fn connected_components(&self) -> Vec<Vec<usize>> {
      let mut components = Vec::new();
      let mut walker = Walker::new();
      for t in 0 .. self.n() {
        if walker.go_next(t) {
          let mut component = vec![];
          while let Some(u) = walker.next() { 
            component.push(u);
            self.each_adjacent_vertex(u, |v| { walker.go_next(v); });
          }
          components.push(component);
        }
      }
      components
    }
    
    /// do BFS
    /// edge is passed to `f`
    /// Note that `from` vertex is not passed to `f`
    fn bfs(&self, from: usize, mut f: impl FnMut(&Self::Edge)) {
      self.walk(from, |walker, u| self.each_edge_from(u, |e| if walker.go_later(self.edge(e).to()) { (f)(self.edge(e)) } ) );
    }
    
    fn shortest_path_bfs<T: Measure>(&self, from: usize) -> Vec<Option<T>> {
      let mut dist = vec![None; self.n()];
      dist[from] = Some(T::zero());
      self.bfs(from, |edge| dist[edge.to()] = dist[edge.from()].map(|d| d + T::one() ) );
      dist
    }

    fn shortest_path_bfs_by<C: Measure>(&self, from: usize, mut f: impl FnMut(&Self::Edge, C) -> Option<C>) -> Vec<Option<C>> {
      let mut dist = vec![None; self.n()];
      dist[from] = Some(C::zero());
      self.walk(from, |walker, u| self.each_edge_from(u, |e| {
        if let Some(d) = (f)(self.edge(e), dist[u].unwrap()) {
          if walker.go_later(self.edge(e).to()) {
            dist[self.edge(e).to()] = Some(d);
          }
        }
      }));
      dist
    }

    /// `f(e, d)? >= d`
    fn shortest_path_dijkstra_by<C: Measure>(&self, from: usize, mut f: impl FnMut(&Self::Edge, C) -> Option<C>) -> Vec<Option<C>> {
      let mut dist = vec![None; self.n()];
      let mut pq = BinaryHeap::new();
      dist[from] = Some(C::zero());
      pq.push((Reverse(C::zero()), from));
      while let Some((Reverse(d1), u)) = pq.pop() {
        if dist[u] != Some(d1) { continue }
        self.each_edge_from(u, |e| if let Some(d) = (f)(self.edge(e), d1) {
          dist[self.edge(e).to()].if_chmin(d, || pq.push((Reverse(d), self.edge(e).to())) );
        } );
      }
      dist
    }

    fn shortest_path_dijkstra(&self, from: usize) -> Vec<Option<E>> where E: Measure {
      self.shortest_path_dijkstra_by(from, |edge, d| Some(d + *edge.weight()) )
    }

    fn shortest_path_spfa_by<C: Measure>(&self, from: usize, mut f: impl FnMut(&Self::Edge, C) -> Option<C>) -> Vec<Option<C>> {
      let mut dist = vec![None; self.n()];
      dist[from] = Some(C::zero());
      let mut q = Uniqueue::new();
      q.push_front(from);
      while let Some(u) = q.pop_back() {
        self.each_edge_from(u, |e| {
          let v = self.edge(e).to();
          if let Some(d) = (f)(self.edge(e), dist[u].unwrap()) {
            dist[v].if_chmin(d, || q.push_front(v) );
          }
        });
      }
      dist
    }

    fn shortest_path_spfa(&self, from: usize) -> Vec<Option<E>> where E: Measure {
      self.shortest_path_spfa_by(from, |edge, d| Some(d + *edge.weight()) )
    }

    fn shortest_paths_floyd_warshall_by<C: Measure>(&self, loops: bool, mut f: impl FnMut(&Self::Edge) -> Option<C>) -> Vec<Vec<Option<C>>> {
      let mut dist = vec![vec![None; self.n()]; self.n()];
      for e in 0 .. self.m() {
        let edge = self.edge(e);
        dist[edge.from()][edge.to()] = (f)(edge);
      }
      if loops {
        for i in 0 .. self.n() {
          dist[i][i] = Some(C::zero());
        }
      }
      for k in 0 .. self.n() {
        for i in 0 .. self.n() {
          for j in 0 .. self.n() {
            if let Some((d1, d2)) = self::measure::zip(dist[i][k], dist[k][j]) {
              dist[i][j].chmin(d1 + d2);
            }
          }
        }
      }
      dist
    }

    fn shortest_paths_floyd_warshall(&self) -> Vec<Vec<Option<E>>> where E: Measure {
      self.shortest_paths_floyd_warshall_by(true, |edge| Some(*edge.weight()) )
    }

    fn minimum_spanning_tree_prim_by<C: Measure, F: FnMut(&Self::Edge) -> Option<C>>(&self, root: usize, mut f: F) -> (C, SubGraph<'_, E, Self>) {
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

    fn minimum_spanning_tree_prim(&self, root: usize) -> (E, SubGraph<'_, E, Self>) where E: Measure {
      self.minimum_spanning_tree_prim_by(root, |edge| Some(*edge.weight()) )
    }
  }
  
  pub trait GraphMut<E = ()>: Graph<E> where Self::Vertex: VertexMut, Self::Edge: EdgeMut<E> {
    fn add_vertex(&mut self) -> usize;
    fn add_vertices(&mut self, n: usize) -> Vec<usize> {
      (0 .. n).map(|_| self.add_vertex() ).collect::<Vec<_>>()
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
    
    fn each_edge_mut_from(&mut self, from: usize, f: impl FnMut(&mut Self::Edge));
    fn each_adjacent_vertex_mut(&mut self, from: usize, f: impl FnMut(&mut Self::Vertex));
  }
  
  pub trait Vertex {}
  
  pub trait Edge<E = ()> {
    fn from(&self) -> usize;
    fn to(&self) -> usize;
    fn weight(&self) -> &E;
  }

  pub trait VertexMut: Vertex {}

  pub trait EdgeMut<E = ()>: Edge<E> {
    fn weight_mut(&mut self) -> &mut E;
  }

  pub trait IntoEdge<E = ()> {
    fn into_edge(self) -> (usize, usize, E);
  }
  
  #[derive(Debug)]
  pub struct Walker {
    visited: FxHashSet<usize>,
    queue: VecDeque<usize>,
  }
  
  pub mod dic_graph {
    use std::marker::PhantomData;
    use std::collections::*;
    use rustc_hash::*;

    use super::{Graph, GraphMut, measure::Measure};

    pub type VecGraph<E = ()> = DicGraph<Vec<Vertex>, E>;
    pub type HashGraph<E = ()> = DicGraph<FxHashMap<usize, Vertex>, E>;
    
    #[derive(Debug, Clone, Default)]
    pub struct Vertex {
      edges: Vec<usize>,
    }
    impl Vertex {
      pub fn edges(&self) -> &Vec<usize> { &self.edges }
    }
    impl super::Vertex for Vertex {}
    impl super::VertexMut for Vertex {}
    
    #[derive(Debug, Clone)]
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
    
    #[derive(Debug, Clone)]
    pub struct DicGraph<D, E = ()> {
      vertices: D,
      edges: Vec<Edge<E>>,
    }
    impl<D: Dic<usize, Vertex>, E> DicGraph<D, E> {
      pub fn new() -> Self {
        Self {
          vertices: D::new(),
          edges: Vec::new(),
        }
      }
    }
    impl<D: Dic<usize, Vertex>, E> Graph<E> for DicGraph<D, E> {
      type Vertex = Vertex;
      type Edge = Edge<E>;
      
      fn n(&self) -> usize { self.vertices.len() }
      fn m(&self) -> usize { self.edges.len() }
      
      fn vertex(&self, id: usize) -> &Vertex {
        assert!(self.vertices.has(&id));
        self.vertices.get(&id).as_ref().unwrap()
      }

      fn edge(&self, id: usize) -> &Edge<E> {
        assert!(id < self.m());
        &self.edges[id]
      }
      
      fn edges_from(&self, from: usize) -> Vec<usize> {
        self.vertex(from).edges.clone()
      }
      
      fn each_edge_from(&self, from: usize, mut f: impl FnMut(usize)) {
        for &e in &self.vertex(from).edges { (f)(e) }
      }
      
      fn adjacent_vertices(&self, from: usize) -> Vec<usize> {
        self.vertex(from).edges.iter().map(|&e| self.edges[e].to ).collect::<Vec<_>>()
      }
      
      fn each_adjacent_vertex(&self, from: usize, mut f: impl FnMut(usize)) {
        for &e in &self.vertex(from).edges { (f)(self.edges[e].to) }
      }

      fn reverse_edge(&self, e: usize) -> Option<usize> { self.edge(e).rev }
    }
    impl<D: Dic<usize, Vertex>, E> GraphMut<E> for DicGraph<D, E> {
      fn add_vertex(&mut self) -> usize {
        let id = self.n();
        self.vertices.insert(id, Vertex { edges: Vec::new() });
        id
      }
      fn add_arc(&mut self, from: usize, to: usize, weight: E) -> usize {
        let id = self.m();
        self.edges.push(Edge { from, to, weight, rev: None });
        self.vertex_mut(from).edges.push(id);
        id
      }

      fn add_edge(&mut self, from: usize, to: usize, weight: E) -> (usize, usize) where E: Clone {
        let id = self.m();
        self.edges.push(Edge { from, to, weight: weight.clone(), rev: Some(id + 1) });
        self.edges.push(Edge { from: to, to: from, weight, rev: Some(id) });
        self.vertex_mut(from).edges.push(id);
        self.vertex_mut(to).edges.push(id + 1);
        (id, id + 1)
      }
      
      fn vertex_mut(&mut self, id: usize) -> &mut Self::Vertex {
        if !self.vertices.has(&id) { self.vertices.insert(id, Vertex { edges: vec![] }) }
        self.vertices.get_mut(&id).unwrap()
      }
      fn edge_mut(&mut self, id: usize) -> &mut Self::Edge {
        assert!(id < self.m());
        &mut self.edges[id]
      }
      
      fn each_edge_mut_from(&mut self, from: usize, mut f: impl FnMut(&mut Self::Edge)) {
        assert!(self.vertices.has(&from));
        for &e in &self.edges_from(from) { (f)(&mut self.edges[e]) }
      }
      fn each_adjacent_vertex_mut(&mut self, from: usize, mut f: impl FnMut(&mut Self::Vertex)) {
        assert!(self.vertices.has(&from));
        for v in self.adjacent_vertices(from) { (f)(self.vertex_mut(v)) }
      }
    }

    pub trait Dic<K, V> {
      fn new() -> Self;
      fn insert(&mut self, key: K, value: V);
      fn get(&self, key: &K) -> Option<&V>;
      fn get_mut(&mut self, key: &K) -> Option<&mut V>;
      fn len(&self) -> usize;
      fn has(&self, key: &K) -> bool;
    }

    impl<K: Eq + std::hash::Hash, V, S: std::hash::BuildHasher + Default> Dic<K, V> for HashMap<K, V, S> {
      fn new() -> Self { Self::default() }
      fn insert(&mut self, key: K, value: V) { HashMap::insert(self, key, value); }
      fn get(&self, key: &K) -> Option<&V> { HashMap::get(self, key) }
      fn get_mut(&mut self, key: &K) -> Option<&mut V> { HashMap::get_mut(self, key) }
      fn len(&self) -> usize { HashMap::len(self) }
      fn has(&self, key: &K) -> bool { self.contains_key(key) }
    }

    impl<T: Default> Dic<usize, T> for Vec<T> {
      fn new() -> Self { Vec::new() }
      fn insert(&mut self, key: usize, value: T) {
        if key >= self.len() { self.resize_with(self.len() * 2 + 1, T::default) };
        self[key] = value;
      }
      fn get(&self, key: &usize) -> Option<&T> { Some(&self[*key]) }
      fn get_mut(&mut self, key: &usize) -> Option<&mut T> { Some(&mut self[*key]) }
      fn len(&self) -> usize { Vec::len(self) }
      fn has(&self, key: &usize) -> bool { *key < self.len() }
    }
  }

  pub mod sub_graph {
    use super::*;
    use std::marker::PhantomData;

    #[derive(Debug)]
    pub struct SubGraph<'a, E, G: Graph<E> + ?Sized> {
      origin: &'a G,
      vertices: Vec<usize>,
      edges: Vec<usize>,
      phantom: PhantomData<E>,
    }
    impl<'a, E, G: Graph<E> + ?Sized> SubGraph<'a, E, G> {
      pub fn new(origin: &'a G, vertices: Vec<usize>, edges: Vec<usize>) -> Self {
        Self { origin, vertices, edges, phantom: PhantomData }
      }
    }
    impl<E, G: Graph<E> + ?Sized> Graph<E> for SubGraph<'_, E, G> {
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
    fn new() -> Self {
      Self {
        visited: FxHashSet::default(),
        queue: VecDeque::new(),
      }
    }
    
    fn go_next(&mut self, v: usize) -> bool {
      if self.visited.insert(v) {
        self.queue.push_back(v);
        return true;
      }
      false
    }
    
    fn go_later(&mut self, v: usize) -> bool {
      if self.visited.insert(v) {
        self.queue.push_front(v);
        return true;
      }
      false
    }
    
    fn forget(&mut self, v: usize) -> bool { self.visited.remove(&v) }

    fn is_visited(&self, v: usize) -> bool { self.visited.contains(&v) }
  }
  impl Iterator for Walker {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> { self.queue.pop_back() }
  }

  impl<E> IntoEdge<E> for (usize, usize, E) {
    fn into_edge(self) -> (usize, usize, E) { self }
  }
  impl<E: Default> IntoEdge<E> for (usize, usize) {
    fn into_edge(self) -> (usize, usize, E) { (self.0, self.1, Default::default()) }
  }
  
  pub mod measure {
    use std::cmp::*;
    use num_traits::*;
    
    pub trait AssignOps: Sized + std::ops::AddAssign + std::ops::SubAssign + std::ops::MulAssign + std::ops::DivAssign + std::ops::RemAssign {}
    pub trait Measure: std::fmt::Debug + Num + Default + Ord + Copy + AssignOps + std::iter::Sum {
      fn chmin(&mut self, other: Self) -> &mut Self {
        if *self > other { *self = other }
        self
      }
      fn chmax(&mut self, other: Self) -> &mut Self {
        if *self < other { *self = other }
        self
      }
      fn if_chmin(&mut self, other: Self, f: impl FnOnce()) -> &mut Self {
        if *self > other {
          *self = other;
          (f)();
        }
        self
      }
      fn if_chmax(&mut self, other: Self, f: impl FnOnce()) -> &mut Self {
        if *self < other {
          *self = other;
          (f)();
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
      fn and_if(self, f: impl FnOnce(T) -> bool) -> bool { self.is_some() && (f)(self.unwrap()) }
      fn if_chmin(&mut self, other: T, f: impl FnOnce()) -> &mut Self where Self: Clone, T: Clone + Ord {
        if !self.is_some() || self.clone().unwrap() > other {
          self.insert(other);
          (f)();
        }
        self
      }
      fn if_chmax(&mut self, other: T, f: impl FnOnce()) -> &mut Self where Self: Clone, T: Clone + Ord {
        if !self.is_some() || self.clone().unwrap() < other {
          self.insert(other);
          (f)();
        }
        self
      }
    }
    impl<T> OptionUtil<T> for Option<T> {
      fn unwrap(self) -> T { Option::<T>::unwrap(self) }
      fn is_some(&self) -> bool { Option::<T>::is_some(self) }
      fn insert(&mut self, value: T) -> &mut T {
        *self = Some(value);
        self.as_mut().unwrap()
      }
    }
    
    impl<T: Sized + std::ops::AddAssign + std::ops::SubAssign + std::ops::MulAssign + std::ops::DivAssign + std::ops::RemAssign> AssignOps for T {}
    impl<T: std::fmt::Debug + Copy + Ord + Default + Num + AssignOps + std::iter::Sum> Measure for T {}
    impl<T: Signed + Measure> MeasureSigned for T {}
    
    pub fn zip<T, U>(left: Option<T>, right: Option<U>) -> Option<(T, U)> {
      left.and_then(|x| right.map(|y| (x, y) ))
    }
  }

  struct Uniqueue<T> {
    queue: VecDeque<T>,
    inq: FxHashSet<T>,
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
  
  use measure::*;
  use std::collections::*;
  use std::cmp::*;
  use itertools::*;
  use rustc_hash::*;
}
