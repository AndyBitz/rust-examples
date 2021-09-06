mod veb;

use veb::VebBase;

fn main() {
    let mut tree = VebBase::new();

    println!("lookup(2) {:?}", tree.lookup(2));

    tree.insert(600);
    tree.insert(700);
    println!("next(1): {}", tree.find_next(1));

    println!("prev(700): {}", tree.find_previous(700));
    println!("prev(32): {}", tree.find_previous(32));
    println!("lookup(600) {:?}", tree.lookup(600));

    tree.delete(600);
    println!("next: {}", tree.find_next(1));

    tree.insert(12);
    tree.insert(1024);
    println!("prev(1025): {}", tree.find_previous(1025));
}
