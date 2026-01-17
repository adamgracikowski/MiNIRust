use red_black_tree::{CharContainer, red_black_tree};

/// Exemplary usage of the Red-Black Tree.
/// Output can be verified using: https://www.cs.usfca.edu/~galles/visualization/RedBlack.html
fn main() {
    println!("--- Red Black Tree ---");

    println!("\n[1] Initializing with 1, 2, 3:");
    let mut tree = red_black_tree! {
        1 => "One",
        2 => "Two",
        3 => "Three"
    };
    tree.print_structure();
    println!();

    println!("\n[2] Inserting 4 and 5:");

    if let Some(s) = CharContainer::new("Four") {
        let _ = tree.insert(4, s);
    }
    tree.print_structure();
    println!();

    if let Some(s) = CharContainer::new("Five") {
        let _ = tree.insert(5, s);
    }
    tree.print_structure();
    println!();

    println!("\n[3] Inserting 6, 7, 8, 9, 10:");
    let values = [
        (6, "Six"),
        (7, "Seven"),
        (8, "Eight"),
        (9, "Nine"),
        (10, "Ten"),
    ];

    for (k, v) in values {
        if let Some(s) = CharContainer::new(v) {
            let _ = tree.insert(k, s);
            tree.print_structure();
            println!();
        }
    }

    println!("\n[4] Verification check:");
    let check_key = 7;
    if let Some(val) = tree.get(check_key) {
        println!("\tTree correctly contains key {check_key}: '{val}'");
    } else {
        println!("\tError: Key {check_key} missing!");
    }

    println!("\n[5] Removing node (key 4):");
    let _ = tree.remove(4);
    tree.print_structure();
    println!();

    println!("\n[6] Removing node (key 2):");
    let _ = tree.remove(2);
    tree.print_structure();
    println!();

    println!("\n[7] Removing node (key 10):");
    let _ = tree.remove(10);
    tree.print_structure();
    println!();
}
