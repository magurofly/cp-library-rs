pub trait Edge<E>: Sized {
  fn to(&self) -> usize;
  fn weight(&self) -> &E;
}

impl Edge<usize> for usize {
  fn to(&self) -> usize { *self }
  fn weight(&self) -> &usize { &1 }
}

impl<E> Edge<E> for (usize, E) {
  fn to(&self) -> usize { self.0 }
  fn weight(&self) -> &E { &self.1 }
}

#[derive(Debug, Clone, Copy)]
pub struct EdgeData<'a, E>(usize, &'a E);
impl<E> Edge<E> for EdgeData<'_, E> {
  fn to(&self) -> usize { self.0 }
  fn weight(&self) -> &E { self.1 }
}

pub struct NamoriGraph {
    cycle: Vec<usize>,
    trees: Vec<Vec<usize>>,
    tree_id: Vec<usize>,
}

impl NamoriGraph {
    pub fn new<E>(graph: &[Vec<impl Edge<E>>]) -> Self {
        let n = graph.len();
        let mut stack = vec![];
        
        let mut is_tree = vec![false; n];
        let mut in_deg = graph.iter().map(Vec::len).collect::<Vec<_>>();
        for i in 0 .. n {
            if in_deg[i] == 1 {
                is_tree[i] = true;
                stack.push(i);
            }
        }
        while let Some(u) = stack.pop() {
            for e in &graph[u] {
                if is_tree[e.to()] {
                    continue;
                }
                in_deg[e.to()] -= 1;
                if in_deg[e.to()] > 1 {
                    continue;
                }
                is_tree[e.to()] = true;
                stack.push(e.to());
            }
        }
        
        let mut cycle = vec![];
        let cycle_first = (0 .. n).find(|&i| !is_tree[i]).unwrap();
        let mut visited = is_tree.clone();
        visited[cycle_first] = true;
        stack.push(cycle_first);
        while let Some(u) = stack.pop() {
            cycle.push(u);
            for e in &graph[u] {
                if visited[e.to()] {
                    continue;
                }
                visited[e.to()] = true;
                stack.push(e.to());
            }
        }
        
        let mut trees = vec![];
        let mut tree_id = vec![n; n];
        for (id, &i) in cycle.iter().enumerate() {
            stack.push(i);
            let mut tree = vec![];
            while let Some(u) = stack.pop() {
                tree.push(u);
                tree_id[u] = id;
                for e in &graph[u] {
                    if !is_tree[e.to()] {
                        continue;
                    }
                    is_tree[e.to()] = false;
                    stack.push(e.to());
                }
            }
            trees.push(tree);
        }
        
        Self { cycle, trees, tree_id }
    }
    
    pub fn cycle(&self) -> &[usize] {
        &self.cycle
    }
    
    pub fn trees(&self) -> usize {
        self.trees.len()
    }
    
    pub fn tree(&self, id: usize) -> &[usize] {
        &self.trees[id]
    }
    
    pub fn tree_id(&self, v: usize) -> usize {
        self.tree_id[v]
    }
}
