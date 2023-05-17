/*
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
*/

/*
struct Custom {
    age: usize,
    name: String,
}

enum Item {
    Number(usize),
    String(String),
    MyCustom(Custom),
}

fn appen(items: &mut Vec<Item>) {
    items.push(Item::String("Hello fem!".into()));
}
*/

/*
fn multiply(value: Option<usize>) -> Option<usize> {
    // return value.unwrap_or(0) * 5;
    return Some(value? * 5);
}
*/

/*
fn practice(nums: Vec<usize>, index: usize) -> usize {
    return nums.get(index).unwrap_or(&index) * 5;
}
*/

fn main() {
    /*
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
    */
    /*
    let mut items: Vec<Item> = vec![];
    appen(&mut items);

    // we cannot do this
    // let mut items: Vec<usize> = vec![];
    // appen(&mut items);

    let foo = Item::Number(5);

    match &foo {
        Item::Number(num) => println!("I am a number: {}", num),
        Item::String(str) => println!("I am a string: {}", str),
        Item::MyCustom(custom) => println!("name: {}, age: {}", custom.name, custom.age),
    }

    match &foo {
        Item::MyCustom(custom) => println!("name: {}, age: {}", custom.name, custom.age),
        _ => {}
    }

    match &foo {
        Item::MyCustom(Custom { age, .. }) => println!("age: {}", age),
        _ => {}
    }

    match &foo {
        Item::MyCustom(custom) if custom.name == "Ricky" => println!("Hi, Ricky"),
        Item::MyCustom(custom) if custom.age > 33 => println!("Jesus age < of this one"),
        Item::MyCustom(custom) if custom.age < 30 => println!("Hello 30"),
        _ => {}
    }
    */

    // println!("{:?}", practice(vec![1, 2], 6));

    let file_name = std::env::args()
        .nth(1)
        .expect("The file name to be passed in");

    let file = std::fs::read_to_string(file_name).expect("unable to read the file to string");

    file.lines().for_each(|line| {
        if let Ok(value) = line.parse::<usize>() {
            println!("{}", value);
        } else {
            println!("Line not a number");
        }
    });
}
