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
    let first = items.first_mut();
    println!("{:?}", first);

    print_all(&items);

    // This is not possible because we passed to print all a mutable reference of items
    // but we don't know if first is till there
    println!("{:?}", first);
}
