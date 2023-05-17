fn main() {
    // this is not working because the vec is living only inside this line
    let items = vec![1, 2, 3].iter().map(|x| x + 1);
    println!("{:?}", items);

    // first we need to assign the vec to a variable, in order to own the value
    let data = vec![1, 2, 3];

    // and with data we can mutate the value
    let items = data.iter().map(|x| x + 1);

    println!("{:?}", items);
}
