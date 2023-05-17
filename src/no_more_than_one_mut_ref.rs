#[derive(Debug)]
struct Item {
    count: usize,
}

fn print_all(items: &Vec<Item>) {
    for item in items {
        println!("{:?}", item);
    }
}

fn main() {
    // this is the borrow of the item, and it is mutable
    let mut items = vec![Item { count: 0 }];
    let first = items.get_mut(0);
    let second = items.get_mut(1);

    // we cannot have more than one mutable reference
    println!("{:?}", first);
}
