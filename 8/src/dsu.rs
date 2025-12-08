// without height heuristic
pub struct Dsu {
    pub parents: Vec<usize>,
    pub sizes: Vec<usize>,
}

impl Dsu {
    pub fn new(size: usize) -> Self {
        let parents = (0..size).collect();
        let sizes = (0..size).map(|_| 1).collect();
        Self { parents, sizes }
    }
    
    // root of a becomes parent of b
    pub fn union(&mut self, a: usize, b: usize) {
        let a_parent = self.parent(a);
        let b_parent = self.parent(b);
        self.parents[b_parent] = a_parent;
        self.sizes[a_parent] += self.sizes[b_parent];
    }
    
    pub fn parent(&mut self, i: usize) -> usize {
        let p = self.parents[i];
        if i != p {
            let root_parent = self.parent(p);
            self.parents[i] = root_parent;
            root_parent
        } else {
            i
        }
    }
    
    pub fn is_same_set(&mut self, a: usize, b: usize) -> bool {
        self.parent(a) == self.parent(b)
    }
    
    pub fn size(&mut self, i: usize) -> usize {
        let p = self.parent(i);
        self.sizes[p]
    }
    
    // size of each connected component
    pub fn sizes(&self) -> impl Iterator<Item = usize> {
        (0..self.parents.len())
            .filter(|p| self.parents[*p] == *p)
            .map(|p| self.sizes[p])
    }
}