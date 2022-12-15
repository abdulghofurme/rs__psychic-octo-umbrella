use std::env::{args, Args};

fn main() {
    let mut args: Args = args();

    let first_arg = args.nth(1).unwrap();
    let operator = args.next().unwrap().chars().nth(0).unwrap();
    let second_arg = args.next().unwrap();

    let first_number = first_arg.parse::<f32>().unwrap();
    let second_number = second_arg.parse::<f32>().unwrap();
    let result = operate(first_number, operator, second_number);
    println!("{}", output(first_number, operator, second_number, result));
}

fn operate(first_number: f32, operator: char, second_number: f32) -> f32 {
    match operator {
        '+' => first_number + second_number,
        '-' => first_number - second_number,
        '/' => first_number / second_number,
        '*' | 'x' | 'X' => first_number * second_number,
        _ => panic!("Invalid operator used."),
    }
}

fn output(first_number: f32, operator: char, second_number: f32, result: f32) -> String {
    format!("{} {} {} = {}", first_number, operator, second_number, result)
}
