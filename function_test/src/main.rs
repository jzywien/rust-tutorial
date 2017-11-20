use std::io;
mod fibonacci;
mod number;

fn main() {
    let num = get_number_input();
    let suffix = number::get_suffix_for_num(num);
    let fib_n = fibonacci::fib(num);
    println!("{}{} Fibonacci = {}", num, suffix, fib_n)
}

fn get_number_input() -> u32 {
    println!("Enter Number: ");
    let mut num = String::new();

    io::stdin()
        .read_line(&mut num)
        .expect("Failed to read line!");

    match num.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            panic!("Please type a number!");
        }
    }
}

