use std::io::{self, Write};
fn read_int() -> i32 {
    let mut num = String::new();

    io::stdin()
        .read_line(&mut num)
        .expect("Error reading number");

    num.trim()
        .parse::<i32>()
        .unwrap()
}

fn main() {
    println!("\n");
    println!("\tAddition of Two Numbers in Rust");
    println!("\n");
    let a = read_int();
    println!("Enter second number ? ");
    let b = read_int();
    println!("\n 1 for sum\n2 for sub\n3 for multi\n4 for div");
    let choice = read_int();

    match choice {
        1 => println!("sum={}",a+b),
        2 => println!("sub={}",a-b),
        3 => println!("div={}",a/b),
        4 => println!("mul={}",a*b),
        _ => println!("Enter only default number"),
    }
}