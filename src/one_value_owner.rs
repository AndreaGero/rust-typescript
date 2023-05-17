#[derive(Debug)]
struct Item {
    count: usize,
}

// This function is mutating the item, so we must
// Pass the mutable reference of the item
// the reference is because we are just mutating
// and we cannot borrow the item itself
fn add_one(item: &mut Item) {
    item.count += 1;
}

fn main() {
    // this is the borrow of the item, and it is mutable
    let mut item = Item { count: 0 };

    println!("{:?}", item);

    // here we are passing the mutable reference of the item in order to
    // changin it without leaving the borrow to the function
    add_one(&mut item);

    println!("{:?}", item);
}
