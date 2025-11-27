use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

pub struct Vertex {
    pub out_edge_owned: Vec<Rc<RefCell<Vertex>>>,
    pub out_edges: Vec<Weak<RefCell<Vertex>>>,
    pub data: i32,
}

impl Vertex {
    pub fn new(data: i32) -> Self {
        Vertex {
            out_edge_owned: Vec::new(),
            out_edges: Vec::new(),
            data,
        }
    }

    pub fn create_neighbour(&mut self) -> Rc<RefCell<Vertex>> {
        let neighbour = Rc::new(RefCell::new(Vertex::new(0)));
        self.out_edge_owned.push(Rc::clone(&neighbour));
        neighbour
    }

    pub fn link_to(&mut self, other: &Rc<RefCell<Vertex>>) {
        self.out_edges.push(Rc::downgrade(other));
    }

    pub fn all_neighbours(&self) -> Vec<Weak<RefCell<Vertex>>> {
        let mut neighbours = Vec::new();

        for neighbour in &self.out_edge_owned {
            neighbours.push(Rc::downgrade(neighbour));
        }

        for weak_neighbour in &self.out_edges {
            neighbours.push(weak_neighbour.clone());
        }

        neighbours
    }

    pub fn cycle(n: usize) -> Rc<RefCell<Vertex>> {
        let first = Rc::new(RefCell::new(Vertex::new(0)));
        let mut current = Rc::clone(&first);
        for i in 1..n {
            let next = current.borrow_mut().create_neighbour();
            next.borrow_mut().data = i as i32;
            current = next;
        }
        current.borrow_mut().link_to(&first);
        first
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use super::Vertex;

    #[test]
    fn test_vertex_cycle() {
        let cycle_start = Vertex::cycle(5);
        let mut current = Rc::clone(&cycle_start);
        for i in 0..5 {
            assert_eq!(current.borrow().data, i as i32);
            let neighbours = current.borrow().all_neighbours();
            assert_eq!(neighbours.len(), 1);
            let next_weak = &neighbours[0];
            let next_rc = next_weak.upgrade().unwrap();
            current = next_rc;
        }
        assert!(Rc::ptr_eq(&current, &cycle_start));
    }
}
