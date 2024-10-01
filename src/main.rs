use std::io;
use std::process::Command;
use std::result;


// Steps 1 D, 2
fn main() {
    println!("Write your operation type between");
    println!("| + | - | * | / |");

    println!("Enter the first value:");
    let f_value = gotNumber();

    println!("Enter the second value:");
    let s_value = gotNumber();

    let operation = Operation::Add(10.0, 5.0);
    calculate(operation);
     
}

enum Operation {
    Add (f64, f64 ),
    Substract (f64, f64 ),
    Multiply (f64, f64 ),
    Divide (f64,f64 ),
}

fn gotNumber () -> f64 {
    let mut input = String::new();
    println!("Enter a number:");
    io::stdin().read_line(&mut input).unwrap();
    let num: f64 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Please enter a number.");
            return 0.0;
        }
    };
    num
}

fn setOperation() -> Result<String, String>{
    let mut input = String::new();
    println!("Enter an operator:");
    io::stdin().read_line(&mut input).unwrap();
    let operation =  input.trim();

    match operation {
        "+" => Ok("Add".to_string()),
        "-" => Ok("Substract".to_string()),
        "*" => Ok( "Multiply".to_string()),
        "/" => Ok("Divide".to_string()),
        _ => Err("Invalid input. Please enter a valid operator.".to_string()),
    }
}

fn calculate(op: Operation) {;
    match op {
        Operation::Add(x, y) => {
            let mut result = x + y;
            println!("Move to coordinates x: {}, y: {}, result: {}", x, y, result);
        }
        Operation::Substract(x, y) => {
            println!("Move to coordinates x: {}, y: {}", x, y);
        }
        Operation::Multiply(x, y) => {
            println!("Move to coordinates x: {}, y: {}", x, y);
        }
        Operation::Divide(x, y) => {
            println!("Move to coordinates x: {}, y: {}", x, y);
        }
    }
}

// Testing

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_get_num() {


// }