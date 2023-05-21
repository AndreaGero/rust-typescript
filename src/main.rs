mod shapes;

use std::{fmt::Display, str::FromStr};

use anyhow::Result;
use shapes::{
    circle::Circle,
    collisions::{Collidable, Contains, Points},
    rect::Rect,
};

enum Shape {
    Circle(Circle),
    Rect(Rect),
}

impl FromStr for Shape {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let (shape, data) = s.split_once(" ").unwrap_or(("", ""));

        match shape {
            "rect" => return Ok(Shape::Rect(data.parse()?)),
            "circle" => return Ok(Shape::Circle(data.parse()?)),
            _ => return Err(anyhow::anyhow!("bad shape")),
        }
    }
}

impl Points for &Shape {
    fn points(&self) -> shapes::collisions::PointIter {
        match self {
            Shape::Circle(c) => c.points(),
            Shape::Rect(r) => r.points(),
        }
    }
}

impl Contains for &Shape {
    fn contains_points(&self, point: (f64, f64)) -> bool {
        match self {
            Shape::Circle(c) => c.contains_points(point),
            Shape::Rect(r) => r.contains_points(point),
        }
    }
}

impl Display for Shape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Shape::Circle(c) => write!(f, "{}", c),
            Shape::Rect(r) => write!(f, "{}", r),
        }
    }
}

fn main() -> Result<()> {
    let shapes = std::fs::read_to_string("shapes.txt")?
        .lines()
        .filter_map(|x| x.parse::<Shape>().ok())
        .collect::<Vec<_>>();

    shapes
        .iter()
        .skip(1)
        .zip(shapes.iter().take(shapes.len() - 1))
        .filter(|(a, b)| a.collide(b))
        .for_each(|(a, b)| println!("{} collides with {}", a, b));

    return Ok(());
}
