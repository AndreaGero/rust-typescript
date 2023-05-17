fn main() {
    // first we need to assign the vec to a variable, in order to own the value
    let data = vec![1, 2, 3];

    // and with data we can mutate the value
    let items = data.iter().map(|x| x + 1);

    println!("{:?}", items);
}
