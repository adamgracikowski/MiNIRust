mod austro_hungarian_greeter;
mod cannon_head;
mod files;
mod graphs;
mod heap_or_stack;

use austro_hungarian_greeter::AustroHungarianGreeter;
use cannon_head::canon_head;
use graphs::Vertex;
use heap_or_stack::HeapOrStack;

use std::{borrow::Cow, cell::RefCell, collections::VecDeque, rc::Rc};

fn main() {
    // Part 1
    {
        let greeter = AustroHungarianGreeter::new();
        for _ in 0..10 {
            println!("{}", greeter.greet());
        }
    }

    // Part 2
    let stack_value = HeapOrStack::Stack(10);
    let heap_value = HeapOrStack::Heap(Box::new(20));

    println!("Stack value: {}", *stack_value);
    println!("Heap value: {}", *heap_value);

    // Part 3
    let xs = VecDeque::from(vec![2, 4, 5, 6]);
    let result = canon_head(&xs).unwrap();
    match result {
        Cow::Borrowed(borrowed) => println!("Borrowed: {borrowed:?}"),
        Cow::Owned(owned) => println!("Owned: {owned:?}"),
    }

    // Part 5
    println!("--- Testing cycle() method ---");
    let node_0 = Vertex::cycle(3);
    println!("Cycle created starting at vertex: {}", node_0.borrow().data);

    let node_1 = node_0.borrow().out_edge_owned[0].clone();
    let node_2 = node_1.borrow().out_edge_owned[0].clone();

    println!(
        "Cycle structure: {} -> {} -> {}",
        node_0.borrow().data,
        node_1.borrow().data,
        node_2.borrow().data
    );

    println!("\n--- Testing create_neighbour() method ---");
    let branch_node = node_1.borrow_mut().create_neighbour();
    branch_node.borrow_mut().data = 99;
    println!("Added strong neighbor '99' to node '1'");

    println!("\n--- Testing new() and link_to() methods ---");
    let observer = Rc::new(RefCell::new(Vertex::new(100)));
    println!("Created independent vertex: {}", observer.borrow().data);

    observer.borrow_mut().link_to(&node_2);
    println!("Vertex 100 linked (weakly) to Vertex 2");

    println!("\n--- Testing all_neighbours() method ---");

    println!("Neighbors of Node 1 (Expected: 2 and 99):");
    let neighbours_of_1 = node_1.borrow().all_neighbours();
    for weak in neighbours_of_1 {
        if let Some(rc) = weak.upgrade() {
            println!(" -> Found neighbor: {}", rc.borrow().data);
        }
    }

    println!("Neighbors of Node 2 (Expected: 0 [cycle back-link]):");
    let neighbours_of_2 = node_2.borrow().all_neighbours();
    for weak in neighbours_of_2 {
        if let Some(rc) = weak.upgrade() {
            println!(" -> Found neighbor: {}", rc.borrow().data);
        }
    }

    println!("Neighbors of Node 100 (Expected: 2):");
    let neighbours_of_observer = observer.borrow().all_neighbours();
    for weak in neighbours_of_observer {
        if let Some(rc) = weak.upgrade() {
            println!(" -> Found neighbor: {}", rc.borrow().data);
        }
    }
}
