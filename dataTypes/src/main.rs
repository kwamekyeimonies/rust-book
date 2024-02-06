use std::io;

fn main() {
    let fruits=["watermelon","banana","coconut"];

    println!("Please enter the index of the array");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read console line");

    let index: usize = index.trim().parse().expect("Index entered wasn't a number");

    let element = fruits[index];

    println!("The value of the element at the index {index} is  {element}");
}
