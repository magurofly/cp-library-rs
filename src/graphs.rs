pub mod graphs {
  // Last Update: 2021-06-23 02:34
  #![allow(unused_imports, dead_code)]
  
  pub use dic_graph::{DicGraph, VecGraph, HashGraph};
  pub use sub_graph::SubGraph;
  pub use grid_graph::GridGraph;

  pub trait VertexId: Copy + Clone + Eq + std::hash::Hash {}
  impl<V: Copy + Clone + Eq + std::hash::Hash> VertexId for V {}
  
  pub trait Graph<V: VertexId, E> {
    type Vertex: Vertex;
    type Edge: Edge<V, E>;
    
    fn vertex(&self, id: V) -> &Self::Vertex;
    fn edge(&self, id: usize) -> &Self::Edge;

    fn each_vertex(&self, f: impl FnMut(V));
    
    /// Number of vertices
    fn n(&self) -> usize;
    
    /// Number of edges
    fn m(&self) -> usize;
    
    fn edges_from(&self, from: V) -> Vec<usize>;
    fn adjacent_vertices(&self, from: V) -> Vec<V>;

    fn each_edge_from(&self, from: V, mut f: impl FnMut(usize)) {
      for e in self.edges_from(from) { (f)(e) }
    }

    fn each_adjacent_vertex(&self, from: V, mut f: impl FnMut(V)) {
      for v in self.adjacent_vertices(from) { (f)(v) }
    }

    fn find_edge(&self, from: V, to: V) -> Option<usize> {
      self.edges_from(from).into_iter().filter(|&e| self.edge(e).to() == to ).next()
    }

    fn reverse_edge(&self, e: usize) -> Option<usize> {
      let edge = self.edge(e);
      self.find_edge(edge.to(), edge.from())
    }
    
    fn walk(&self, from: V, mut f: impl FnMut(&mut Walker<V>, V)) {
      let mut walker = Walker::new();
      walker.go_next(from);
      while let Some(u) = walker.next() { (f)(&mut walker, u) }
    }
    
    /// do DFS in preorder
    /// edge is passed to `f`
    /// Note that `from` vertex is not passed to `f`
    fn dfs(&self, from: V, mut f: impl FnMut(&Self::Edge)) {
      self.walk(from, |walker, u| self.each_edge_from(u, |e| if walker.go_next(self.edge(e).to()) { (f)(self.edge(e)) } ) );
    }

    fn dfs_preorder(&self, from: V, mut f: impl FnMut(V)) {
      (f)(from);
      self.dfs(from, |edge| (f)(edge.to()) );
    }

    // /// do DFS in postorder
    fn dfs_postorder(&self, from: V, mut f: impl FnMut(V)) {
      let mut stack = vec![from];
      let mut visited = FxHashSet::default();
      while let Some(u) = stack.pop() {
        if visited.contains(&u) {
          (f)(u);
        } else {
          visited.insert(u);
          stack.push(u);
          self.each_adjacent_vertex(u, |v| {
            if !visited.contains(&v) { stack.push(v) };
          });
        }
      }
    }

    fn eulertour(&self, from: V, mut f: impl FnMut(&Self::Edge)) {
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
    
    fn connected_components(&self) -> Vec<Vec<V>> {
      let mut components = Vec::new();
      let mut walker = Walker::new();
      self.each_vertex(|t| {
        if walker.go_next(t) {
          let mut component = vec![];
          while let Some(u) = walker.next() { 
            component.push(u);
            self.each_adjacent_vertex(u, |v| { walker.go_next(v); });
          }
          components.push(component);
        }
      });
      components
    }
    
    /// do BFS
    /// edge is passed to `f`
    /// Note that `from` vertex is not passed to `f`
    fn bfs(&self, from: V, mut f: impl FnMut(&Self::Edge)) {
      self.walk(from, |walker, u| self.each_edge_from(u, |e| if walker.go_later(self.edge(e).to()) { (f)(self.edge(e)) } ) );
    }
    
    fn shortest_path_bfs<T: Measure>(&self, from: V) -> FxHashMap<V, T> {
      let mut dist = FxHashMap::default();
      dist.insert(from, T::zero());
      self.bfs(from, |edge| { dist.insert(edge.to(), dist[&edge.from()] + T::one()); });
      dist
    }

    fn shortest_path_bfs_by<C: Measure>(&self, from: V, mut f: impl FnMut(&Self::Edge, C) -> Option<C>) -> FxHashMap<V, C> {
      let mut dist = FxHashMap::default();
      dist.insert(from, C::zero());
      self.walk(from, |walker, u| self.each_edge_from(u, |e| {
        if let Some(d) = (f)(self.edge(e), dist[&u]) {
          if walker.go_later(self.edge(e).to()) {
            dist.insert(self.edge(e).to(), d);
          }
        }
      }));
      dist
    }

    /// `f(e, d)? >= d`
    ///FIXME: なんかこわれてるので直す
    fn shortest_path_dijkstra_by<C: Measure>(&self, from: V, mut f: impl FnMut(&Self::Edge, C) -> Option<C>) -> FxHashMap<V, C> {
      let mut dist = FxHashMap::default();
      dist.insert(from, C::zero());
      let mut pq = BinaryHeap::new();
      pq.push(DistV(C::zero(), from));
      while let Some(DistV(d1, u)) = pq.pop() {
        if dist[&u] != d1 { continue }
        self.each_edge_from(u, |e| if let Some(d) = (f)(self.edge(e), d1) {
          dist.if_chmin(self.edge(e).to(), d, || pq.push(DistV(d, self.edge(e).to())) );
        } );
      }
      dist
    }

    fn shortest_path_dijkstra(&self, from: V) -> FxHashMap<V, E> where E: Measure {
      self.shortest_path_dijkstra_by(from, |edge, d| Some(d + *edge.weight()) )
    }

    fn shortest_path_spfa_by<C: Measure>(&self, from: V, mut f: impl FnMut(&Self::Edge, C) -> Option<C>) -> FxHashMap<V, C> {
      let mut dist = FxHashMap::default();
      dist.insert(from, C::zero());
      let mut q = Uniqueue::new();
      q.push_front(from);
      while let Some(u) = q.pop_back() {
        self.each_edge_from(u, |e| {
          let v = self.edge(e).to();
          if let Some(d) = (f)(self.edge(e), dist[&u]) {
            dist.if_chmin(v, d, || q.push_front(v) );
          }
        });
      }
      dist
    }

    fn shortest_path_spfa(&self, from: V) -> FxHashMap<V, E> where E: Measure {
      self.shortest_path_spfa_by(from, |edge, d| Some(d + *edge.weight()) )
    }

    fn shortest_paths_floyd_warshall_by<C: Measure>(&self, loops: bool, mut f: impl FnMut(&Self::Edge) -> Option<C>) -> FxHashMap<V, FxHashMap<V, C>> {
      let mut dist = FxHashMap::default();
      for e in 0 .. self.m() {
        let edge = self.edge(e);
        if let Some(d) = (f)(edge) { dist.entry(edge.from()).or_insert_with(FxHashMap::default).insert(edge.to(), d); }
      }
      if loops {
        self.each_vertex(|v| {
          dist.entry(v).or_insert_with(FxHashMap::default).insert(v, C::zero());
        });
      }
      self.each_vertex(|k| {
        self.each_vertex(|i| {
          self.each_vertex(|j| {
            if let Some((&d1, &d2)) = self::measure::zip(dist.get(&i).and_then(|d| d.get(&k) ), dist.get(&k).and_then(|d| d.get(&j) )) {
              dist.entry(i).or_insert_with(FxHashMap::default).chmin(j, d1 + d2);
            }
          });
        });
      });
      dist
    }

    fn shortest_paths_floyd_warshall(&self) -> FxHashMap<V, FxHashMap<V, E>> where E: Measure {
      self.shortest_paths_floyd_warshall_by(true, |edge| Some(*edge.weight()) )
    }

    // fn minimum_spanning_tree_prim_by<C: Measure, F: FnMut(&Self::Edge) -> Option<C>>(&self, root: usize, mut f: F) -> (C, SubGraph<'_, V, E, Self>) {
    //   let mut cost = C::zero();
    //   let mut included = vec![false; self.n()];
    //   let mut vertices = vec![root];
    //   let mut edges = Vec::new();
    //   let mut pq = self.edges_from(root).into_iter().filter_map(|e| (f)(self.edge(e)).map(|c| (Reverse(c), e) ) ).collect::<BinaryHeap<_>>();
    //   included[root] = true;
    //   while let Some((Reverse(c), e)) = pq.pop() {
    //     let to = self.edge(e).to();
    //     if included[to] { continue }
    //     included[to] = true;
    //     vertices.push(to);
    //     edges.push(e);
    //     cost += c;
    //     for x in self.edges_from(to).into_iter().filter_map(|e| (f)(self.edge(e)).map(|c| (Reverse(c), e) ) ) {
    //       pq.push(x);
    //     }
    //   }
    //   (cost, SubGraph::new(self, vertices, edges))
    // }

    // fn minimum_spanning_tree_prim(&self, root: usize) -> (E, SubGraph<'_, V, E, Self>) where E: Measure {
    //   self.minimum_spanning_tree_prim_by(root, |edge| Some(*edge.weight()) )
    // }
  }
  
  pub trait GraphMut<V: VertexId, E>: Graph<V, E> where Self::Vertex: VertexMut, Self::Edge: EdgeMut<V, E> {
    fn add_arc(&mut self, from: V, to: V, weight: E) -> usize;
    fn connect(&mut self, from: V, to: V) -> usize where E: Default { self.add_arc(from, to, Default::default()) }
    fn add_edge(&mut self, u: V, v: V, weight: E) -> (usize, usize) where E: Clone { (self.add_arc(u, v, weight.clone()), self.add_arc(v, u, weight)) }
    fn connect2(&mut self, u: V, v: V) -> (usize, usize) where E: Default { (self.connect(u, v), self.connect(v, u)) }

    fn add_arcs<I: IntoIterator>(&mut self, edges: I) -> Vec<usize> where I::Item: IntoEdge<V, E> {
      edges.into_iter().map(|edge| {
        let (from, to, weight) = edge.into_edge();
        self.add_arc(from, to, weight)
      }).collect::<Vec<_>>()
    }
    fn add_edges<I: IntoIterator>(&mut self, edges: I) -> Vec<(usize, usize)> where E: Clone, I::Item: IntoEdge<V, E> {
      edges.into_iter().map(|edge| {
        let (from, to, weight) = edge.into_edge();
        self.add_edge(from, to, weight)
      }).collect::<Vec<_>>()
    }
    
    fn vertex_mut(&mut self, id: V) -> &mut Self::Vertex;
    fn edge_mut(&mut self, id: usize) -> &mut Self::Edge;
    
    fn each_edge_mut_from(&mut self, from: V, f: impl FnMut(&mut Self::Edge));
    fn each_adjacent_vertex_mut(&mut self, from: V, f: impl FnMut(&mut Self::Vertex));
  }
  
  pub trait Vertex {}
  
  pub trait Edge<V, E> {
    fn from(&self) -> V;
    fn to(&self) -> V;
    fn weight(&self) -> &E;
  }

  pub trait VertexMut: Vertex {}

  pub trait EdgeMut<V, E>: Edge<V, E> {
    fn weight_mut(&mut self) -> &mut E;
  }

  pub trait IntoEdge<V, E> {
    fn into_edge(self) -> (V, V, E);
  }
  
  #[derive(Debug)]
  pub struct Walker<V: VertexId> {
    visited: FxHashSet<V>,
    queue: VecDeque<V>,
  }
  
  pub mod dic_graph {
    use std::marker::PhantomData;
    use std::collections::*;
    use rustc_hash::*;

    use super::{Graph, GraphMut, measure::Measure, Dic, VertexId};

    pub type VecGraph<E = ()> = DicGraph<Vec<Option<Vertex>>, usize, E>;
    pub type HashGraph<V, E = ()> = DicGraph<FxHashMap<V, Vertex>, V, E>;
    
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
    pub struct Edge<V, E> {
      from: V,
      to: V,
      weight: E,
      rev: Option<usize>,
    }
    impl<V: VertexId, E> super::Edge<V, E> for Edge<V, E> {
      fn from(&self) -> V { self.from }
      fn to(&self) -> V { self.to }
      fn weight(&self) -> &E { &self.weight }
    }
    impl<V: VertexId, E> super::EdgeMut<V, E> for Edge<V, E> {
      fn weight_mut(&mut self) -> &mut E { &mut self.weight }
    }
    
    #[derive(Debug, Clone)]
    pub struct DicGraph<D, V: VertexId, E> {
      vertices: D,
      edges: Vec<Edge<V, E>>,
    }
    impl<D: Dic<V, Vertex>, V: VertexId, E> DicGraph<D, V, E> {
      pub fn new() -> Self {
        Self {
          vertices: D::new(),
          edges: Vec::new(),
        }
      }
    }
    impl<D: Dic<V, Vertex>, V: VertexId, E> Graph<V, E> for DicGraph<D, V, E> {
      type Vertex = Vertex;
      type Edge = Edge<V, E>;
      
      fn n(&self) -> usize { self.vertices.len() }
      fn m(&self) -> usize { self.edges.len() }

      fn each_vertex(&self, mut f: impl FnMut(V)) { self.vertices.each_key(|&v| (f)(v) ) }
      
      fn vertex(&self, id: V) -> &Vertex {
        assert!(self.vertices.has(&id));
        self.vertices.get(&id).as_ref().unwrap()
      }

      fn edge(&self, id: usize) -> &Self::Edge {
        assert!(id < self.m());
        &self.edges[id]
      }
      
      fn edges_from(&self, from: V) -> Vec<usize> {
        self.vertex(from).edges.clone()
      }
      
      fn each_edge_from(&self, from: V, mut f: impl FnMut(usize)) {
        if !self.vertices.has(&from) { return };
        for &e in &self.vertex(from).edges { (f)(e) }
      }
      
      fn adjacent_vertices(&self, from: V) -> Vec<V> {
        if !self.vertices.has(&from) { return Vec::new() };
        self.vertex(from).edges.iter().map(|&e| self.edges[e].to ).collect::<Vec<_>>()
      }
      
      fn each_adjacent_vertex(&self, from: V, mut f: impl FnMut(V)) {
        if !self.vertices.has(&from) { return };
        for &e in &self.vertex(from).edges { (f)(self.edges[e].to) }
      }

      fn reverse_edge(&self, e: usize) -> Option<usize> { self.edge(e).rev }
    }
    impl<D: Dic<V, Vertex>, V: VertexId, E> GraphMut<V, E> for DicGraph<D, V, E> {
      fn add_arc(&mut self, from: V, to: V, weight: E) -> usize {
        let id = self.m();
        self.edges.push(Edge { from, to, weight, rev: None });
        self.vertex_mut(from).edges.push(id);
        id
      }

      fn add_edge(&mut self, from: V, to: V, weight: E) -> (usize, usize) where E: Clone {
        let id = self.m();
        self.edges.push(Edge { from, to, weight: weight.clone(), rev: Some(id + 1) });
        self.edges.push(Edge { from: to, to: from, weight, rev: Some(id) });
        self.vertex_mut(from).edges.push(id);
        self.vertex_mut(to).edges.push(id + 1);
        (id, id + 1)
      }
      
      fn vertex_mut(&mut self, id: V) -> &mut Self::Vertex {
        if !self.vertices.has(&id) { self.vertices.insert(id, Vertex { edges: vec![] }) }
        self.vertices.get_mut(&id).unwrap()
      }
      fn edge_mut(&mut self, id: usize) -> &mut Self::Edge {
        assert!(id < self.m());
        &mut self.edges[id]
      }
      
      fn each_edge_mut_from(&mut self, from: V, mut f: impl FnMut(&mut Self::Edge)) {
        assert!(self.vertices.has(&from));
        for &e in &self.edges_from(from) { (f)(&mut self.edges[e]) }
      }
      fn each_adjacent_vertex_mut(&mut self, from: V, mut f: impl FnMut(&mut Self::Vertex)) {
        assert!(self.vertices.has(&from));
        for v in self.adjacent_vertices(from) { (f)(self.vertex_mut(v)) }
      }
    }
  }

  pub mod sub_graph {
    use super::{Graph, VertexId, Edge as _};
    use rustc_hash::FxHashMap;
    use std::marker::PhantomData;

    #[derive(Debug)]
    pub struct SubGraph<'a, V: VertexId, E, G: Graph<V, E>> {
      origin: &'a G,
      vertices: Vec<V>,
      vertex_map: FxHashMap<V, usize>,
      edges: Vec<Edge<'a, V, E, G>>,
      edge_map: FxHashMap<usize, usize>,
    }
    impl<'a, V: VertexId, E, G: Graph<V, E>> SubGraph<'a, V, E, G> {
      pub fn new(origin: &'a G, vertices: Vec<V>, edges: Vec<usize>) -> Self {
        let mut vertex_map = FxHashMap::default();
        for (u, &v) in vertices.iter().enumerate() {
          vertex_map.insert(v, u);
        }
        let edges = edges.into_iter().map(|e| {
          let edge = origin.edge(e);
          Edge { graph: origin, from: vertex_map[&edge.from()], to: vertex_map[&edge.to()], id: e, phantom: PhantomData }
        }).collect::<Vec<_>>();
        let mut edge_map = FxHashMap::default();
        for (e, edge) in edges.iter().enumerate() {
          edge_map.insert(edge.id, e);
        }
        Self { origin, vertices, vertex_map, edges, edge_map }
      }

      pub fn lookup_vertex(&self, id: V) -> Option<usize> { self.vertex_map.get(&id).copied() }
      pub fn lookup_edge(&self, id: usize) -> Option<usize> { self.edge_map.get(&id).copied() }
    }
    impl<'a, V: VertexId, E, G: Graph<V, E>> Graph<usize, E> for SubGraph<'a, V, E, G> {
      type Vertex = G::Vertex;
      type Edge = Edge<'a, V, E, G>;

      fn n(&self) -> usize { self.vertices.len() }
      fn m(&self) -> usize { self.edges.len() }

      fn each_vertex(&self, mut f: impl FnMut(usize)) { for i in 0 .. self.n() { (f)(i) } }

      fn vertex(&self, id: usize) -> &Self::Vertex {
        assert!(id < self.n());
        self.origin.vertex(self.vertices[id])
      }

      fn edge(&self, id: usize) -> &Self::Edge {
        use super::Edge as _;
        assert!(id < self.m());
        &self.edges[id]
      }

      fn edges_from(&self, from: usize) -> Vec<usize> {
        assert!(from < self.n());
        self.origin.edges_from(self.vertices[from]).into_iter().filter_map(|e| self.lookup_edge(e) ).collect::<Vec<_>>()
      }

      fn adjacent_vertices(&self, from: usize) -> Vec<usize> {
        assert!(from < self.n());
        self.origin.adjacent_vertices(self.vertices[from]).into_iter().filter_map(|v| self.lookup_vertex(v) ).collect::<Vec<_>>()
      }
    }

    #[derive(Clone, Copy, Debug)]
    pub struct Edge<'a, V, E, G> {
      graph: &'a G,
      from: usize,
      to: usize,
      id: usize,
      phantom: PhantomData<(V, E)>
    }
    impl<V: VertexId, E, G: Graph<V, E>> super::Edge<usize, E> for Edge<'_, V, E, G> {
      fn from(&self) -> usize { self.from }
      fn to(&self) -> usize { self.to }
      fn weight(&self) -> &E { self.graph.edge(self.id).weight() }
    }
  }

  pub mod grid_graph {
    pub struct GridGraph {
      rows: usize,
      columns: usize,
      grid: Vec<Vec<char>>,
    }

    impl GridGraph {
      pub fn new(rows: usize, columns: usize, grid: Vec<Vec<char>>) -> Self { Self { rows, columns, grid } }
      pub fn rows(&self) -> usize { self.rows }
      pub fn columns(&self) -> usize { self.columns }
      pub fn valid_vertex(&self, v: (usize, usize)) -> bool { v.0 < self.rows && v.1 < self.columns }
    }

    impl super::Vertex for char {}

    impl super::Graph<(usize, usize), ()> for GridGraph {
      type Vertex = char;
      type Edge = Edge;

      fn n(&self) -> usize { self.rows * self.columns }
      fn m(&self) -> usize { (self.rows * self.columns - self.rows - self.columns) * 2 }

      fn each_vertex(&self, mut f: impl FnMut((usize, usize))) { for i in 0 .. self.rows { for j in 0 .. self.columns { (f)((i, j)) } } }

      fn vertex(&self, id: (usize, usize)) -> &Self::Vertex {
        assert!(self.valid_vertex(id));
        &self.grid[id.0][id.1]
      }
      fn edge(&self, e: usize) -> &Self::Edge {
        assert!(e < self.m());
        let f = e >> 1;
        let (mut from, mut to) = if f < self.rows * (self.columns - 1) {
          let (i, j) = (f / (self.columns - 1), f % (self.columns - 1));
          ((i, j), (i, j + 1))
        } else {
          let f = f - self.rows * (self.columns - 1);
          let (i, j) = (f / self.columns, f / self.columns);
          ((i, j), (i + 1, j))
        };
        if (e & 1) == 1 {
          std::mem::swap(&mut from, &mut to);
        }
        Box::leak(Box::new(Edge { from, to }))
      }

      fn edges_from(&self, from: (usize, usize)) -> Vec<usize> {
        assert!(self.valid_vertex(from));
        let mut edges = vec![];
        let (i, j) = from;
        if j + 1 < self.columns { edges.push(i * (self.columns - 1) + (j - 1) << 1 | 0) }
        if j > 1 { edges.push(i * (self.columns - 1) + j << 1 | 1) }
        if i > 1 { edges.push(self.rows * (self.columns - 1) + i * self.columns + j << 1 | 0) }
        if i + 1 < self.rows { edges.push(self.rows * (self.columns - 1) + (i - 1) * self.columns + j << 1 | 0) }
        edges
      }

      fn adjacent_vertices(&self, from: (usize, usize)) -> Vec<(usize, usize)> {
        assert!(self.valid_vertex(from));
        let mut vertices = vec![];
        let (i, j) = from;
        if j + 1 < self.columns { vertices.push((i, j + 1)) }
        if j > 1 { vertices.push((i, j - 1)) }
        if i > 1 { vertices.push((i + 1, j)) }
        if i + 1 < self.rows { vertices.push((i - 1, j)) }
        vertices
      }
    }

    pub struct Edge {
      from: (usize, usize),
      to: (usize, usize),
    }
    impl super::Edge<(usize, usize), ()> for Edge {
      fn from(&self) -> (usize, usize) { self.from }
      fn to(&self) -> (usize, usize) { self.to }
      fn weight(&self) -> &() { &() }
    }
  }
  
  impl<V: VertexId> Walker<V> {
    pub fn new() -> Self { Self { visited: FxHashSet::default(), queue: VecDeque::new() } }
    pub fn go_next(&mut self, v: V) -> bool { self.visited.insert(v) && { self.queue.push_back(v); true } }
    pub fn go_later(&mut self, v: V) -> bool { self.visited.insert(v) && { self.queue.push_front(v); true } }
    pub fn forget(&mut self, v: V) -> bool { self.visited.remove(&v) }
    pub fn is_visited(&self, v: V) -> bool { self.visited.contains(&v) }
  }
  impl<V: VertexId> Iterator for Walker<V> {
    type Item = V;
    fn next(&mut self) -> Option<Self::Item> { self.queue.pop_back() }
  }

  impl<V, E> IntoEdge<V, E> for (V, V, E) {
    fn into_edge(self) -> (V, V, E) { self }
  }
  impl<V, E: Default> IntoEdge<V, E> for (V, V) {
    fn into_edge(self) -> (V, V, E) { (self.0, self.1, Default::default()) }
  }
  
  pub mod measure {
    use std::cmp::*;
    use num_traits::*;
    
    pub trait AssignOps: Sized + std::ops::AddAssign + std::ops::SubAssign + std::ops::MulAssign + std::ops::DivAssign + std::ops::RemAssign {}
    pub trait Measure: std::fmt::Debug + Num + Default + Ord + Copy + AssignOps + std::iter::Sum {
      fn if_chmin(&mut self, other: Self, f: impl FnOnce()) -> &mut Self { if *self > other { *self = other; (f)() }; self }
      fn if_chmax(&mut self, other: Self, f: impl FnOnce()) -> &mut Self { if *self < other { *self = other; (f)() }; self }
      fn chmin(&mut self, other: Self) -> &mut Self { self.if_chmin(other, || {}) }
      fn chmax(&mut self, other: Self) -> &mut Self { self.if_chmax(other, || {}) }
    }
    pub trait MeasureSigned: Measure + Signed {}
    
    pub trait OptionUtil<T>: Sized {
      fn unwrap(self) -> T;
      fn borrow(&self) -> &T;
      fn is_some(&self) -> bool;
      fn insert(&mut self, value: T) -> &mut T;
      fn and_if(self, f: impl FnOnce(T) -> bool) -> bool { self.is_some() && (f)(self.unwrap()) }
      fn if_chmin(&mut self, other: T, f: impl FnOnce()) -> &mut Self where T: PartialOrd { if !self.is_some() || self.borrow().gt(&other) { self.insert(other); (f)() }; self }
      fn if_chmax(&mut self, other: T, f: impl FnOnce()) -> &mut Self where T: PartialOrd { if !self.is_some() || self.borrow().lt(&other) { self.insert(other); (f)() }; self }
      fn chmin(&mut self, other: T) -> &mut Self where T: PartialOrd { self.if_chmin(other, || {}) }
      fn chmax(&mut self, other: T) -> &mut Self where T: PartialOrd { self.if_chmax(other, || {}) }
    }
    impl<T> OptionUtil<T> for Option<T> {
      fn unwrap(self) -> T { Option::unwrap(self) }
      fn borrow(&self) -> &T { Option::unwrap(self.as_ref()) }
      fn is_some(&self) -> bool { Option::is_some(self) }
      fn insert(&mut self, value: T) -> &mut T { *self = Some(value); self.as_mut().unwrap() }
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
    fn new() -> Self { Self { queue: VecDeque::new(), inq: FxHashSet::default() } }
    fn push_front(&mut self, value: T) {
      if self.inq.contains(&value) { return };
      self.inq.insert(value.clone());
      self.queue.push_front(value);
    }
    fn pop_back(&mut self) -> Option<T> { self.queue.pop_back().map(|value| { self.inq.remove(&value); value }) }
  }

  #[derive(Copy, Clone)]
  pub struct DistV<C: Ord + Eq, V: PartialEq>(C, V);
  impl<C: Ord + Eq, V: PartialEq> Ord for DistV<C, V> {
    fn cmp(&self, other: &Self) -> Ordering { other.0.cmp(&self.0) }
  }
  impl<C: Ord + Eq, V: PartialEq> PartialOrd for DistV<C, V> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> { other.0.partial_cmp(&self.0) }
  }
  impl<C: Ord + Eq, V: PartialEq> PartialEq for DistV<C, V> {
    fn eq(&self, other: &Self) -> bool { self.0.eq(&other.0) && self.1.eq(&other.1) }
  }
  impl<C: Ord + Eq, V: PartialEq> Eq for DistV<C, V> {}

  pub trait Dic<K, V> {
    fn new() -> Self;
    fn insert(&mut self, key: K, value: V);
    fn get(&self, key: &K) -> Option<&V>;
    fn get_mut(&mut self, key: &K) -> Option<&mut V>;
    fn len(&self) -> usize;
    fn has(&self, key: &K) -> bool;
    fn each_key(&self, f: impl FnMut(&K));
    fn chmin(&mut self, key: K, x: V) -> &mut V where K: Clone, V: Measure { if self.has(&key) { self.get_mut(&key).unwrap().chmin(x) } else { self.insert(key.clone(), x); self.get_mut(&key).unwrap() } }
    fn chmax(&mut self, key: K, x: V) -> &mut V where K: Clone, V: Measure { if self.has(&key) { self.get_mut(&key).unwrap().chmax(x) } else { self.insert(key.clone(), x); self.get_mut(&key).unwrap() } }
    fn if_chmin(&mut self, key: K, x: V, f: impl FnOnce()) where V: Measure { if let Some(y) = self.get_mut(&key) { y.if_chmin(x, f); return }; self.insert(key, x); (f)() }
    fn if_chmax(&mut self, key: K, x: V, f: impl FnOnce()) where V: Measure { if let Some(y) = self.get_mut(&key) { y.if_chmax(x, f); return }; self.insert(key, x); (f)() }
  }

  impl<K: Clone + Eq + std::hash::Hash, V, S: std::hash::BuildHasher + Default> Dic<K, V> for HashMap<K, V, S> {
    fn new() -> Self { Self::default() }
    fn insert(&mut self, key: K, value: V) { HashMap::insert(self, key, value); }
    fn get(&self, key: &K) -> Option<&V> { HashMap::get(self, key) }
    fn get_mut(&mut self, key: &K) -> Option<&mut V> { HashMap::get_mut(self, key) }
    fn len(&self) -> usize { HashMap::len(self) }
    fn has(&self, key: &K) -> bool { self.contains_key(key) }
    fn each_key(&self, mut f: impl FnMut(&K)) { for k in self.keys() { (f)(k) } }
  }

  impl<T: Default> Dic<usize, T> for Vec<Option<T>> {
    fn new() -> Self { Vec::new() }
    fn insert(&mut self, key: usize, value: T) {
      if key >= self.len() { self.resize_with(key + 1, || None) };
      self[key] = Some(value);
    }
    fn get(&self, key: &usize) -> Option<&T> { if *key < self.len() { self[*key].as_ref() } else { None } }
    fn get_mut(&mut self, key: &usize) -> Option<&mut T> { if *key < self.len() { self[*key].as_mut() } else { None } }
    fn len(&self) -> usize { Vec::len(self) }
    fn has(&self, key: &usize) -> bool { *key < self.len() && self.get(key).is_some() }
    fn each_key(&self, mut f: impl FnMut(&usize)) { for i in 0 .. self.len() { (f)(&i) } }
  }
  
  use measure::*;
  use std::collections::*;
  use std::cmp::*;
  use itertools::*;
  use rustc_hash::*;
}
