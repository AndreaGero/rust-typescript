enum Color {
    Red,
    Green,
    Blue,
    Yellow,
}

impl Color {
    fn is_green(&self) -> bool {
        if let Color::Green = self {
            return true;
        }
        return false;
    }

    fn is_green_parts(&self) -> bool {
        match self {
            Color::Red => return false,
            Color::Green => return false,
            Color::Blue => return true,
            Color::Yellow => return true,
        }
    }
}

fn print_color(color: Color) {
    match color {
        Color::Red => println!("red"),
        Color::Green => println!("green"),
        Color::Blue => println!("bluw"),
        Color::Yellow => println!("yellow"),
    }
}

fn main() {
    // map
    let data = vec![1, 2, 3];
    let mut list = data.iter().map(|x| x + 1);

    let mut new_vector = vec![];

    while let Some(x) = list.next() {
        new_vector.push(x);
    }

    println!("{:?}", new_vector);

    // read file
    let file = std::fs::read_to_string("lines.txt").unwrap();

    file.lines()
        .enumerate()
        .filter(|(idx, _)| idx % 2 == 0)
        .skip(2)
        .take(2)
        .for_each(|(_, line)| println!("{}", line));

    print_color(Color::Blue);

    let foo = Color::Green;
    foo.is_green();
    foo.is_green_parts();
}
