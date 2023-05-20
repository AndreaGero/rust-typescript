use crate::shapes::{circle::Circle, rect::Rect};

mod shapes;

fn main() {
    let rect = Rect::default();
    let circ = Circle::default();

    println!("{}", rect);
    println!("{}", circ);
}
