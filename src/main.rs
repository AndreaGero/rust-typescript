use crate::shapes::{circle::Circle, collisions::Collidable, rect::Rect};

mod shapes;

fn main() {
    let rect = Rect::default();
    let rect2 = Rect::default();

    let circ = Circle::default();
    let circ2 = Circle::default();

    rect.collide(&rect2);
    circ.collide(&circ2);
    rect.collide(&circ);

    println!("{}", rect);
    println!("{}", circ);
}
