use std::io;

fn main() {
    println!("Simple Calculator in Rust");
    println!("Enter the first number:");

    let mut num1 = String::new();
    io::stdin()
        .read_line(&mut num1)
        .expect("Failed to read line");
    let num1: f64 = match num1.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number!");
            return;
        }
    };

    println!("Enter an operator (+, -, *, /):");
    let mut operator = String::new();
    io::stdin()
        .read_line(&mut operator)
        .expect("Failed to read line");
    let operator = operator.trim();

    println!("Enter the second number:");
    let mut num2 = String::new();
    io::stdin()
        .read_line(&mut num2)
        .expect("Failed to read line");
    let num2: f64 = match num2.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number!");
            return;
        }
    };

    let result = match operator {
        "+" => num1 + num2,
        "-" => num1 - num2,
        "*" => num1 * num2,
        "/" => {
            if num2 == 0.0 {
                println!("Error: Division by zero is not allowed!");
                return;
            }
            num1 / num2
        }
        _ => {
            println!("Invalid operator! Please use +, -, * or /.");
            return;
        }
    };

    println!("Result: {} {} {} = {}", num1, operator, num2, result);
}
