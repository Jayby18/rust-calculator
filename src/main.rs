use std::env::{args, Args};

fn main() {
    // Get CLI arguments
    let mut args: Args = args();

    // First argument is location of binary, so skip it
    // Get first user argument, the first number
    let first: String = args.nth(1).unwrap();
    // Note that the elements above are dropped from args
    // Get second user argument, the operator, and convert to char
    let operator: char = args.next().unwrap().chars().next().unwrap();
    // Get third user argument, the second number
    let second: String = args.next().unwrap();

    // Parse strings to numbers
    let first_number = first.parse::<f32>().unwrap();
    let second_number = second.parse::<f32>().unwrap();

    // Perform operation
    let result = operate(&operator, &first_number, &second_number);

    println!("{}", output(&first_number, &operator, &second_number, &result));
}

fn operate(operator: &char, first_number: &f32, second_number: &f32) -> f32 {
    match operator {
        '+' => first_number + second_number,
        '-' => first_number - second_number,
        '/' => first_number / second_number,
        '%' => first_number % second_number,
        '*' | 'x' | 'X' => first_number * second_number,
        _ => panic!("Invalid operator!"),
    }
}

fn output(a: &f32, b: &char, c: &f32, d: &f32) -> String {
    format!("{} {} {} = {}", a, b, c, d)
}