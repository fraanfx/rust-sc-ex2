use std::io;
//use std::process::Command;
// use std::result;


enum Operation {
    Add (f64, f64 ),
    Substract (f64, f64 ),
    Multiply (f64, f64 ),
    Divide (f64,f64 ),
}

// Steps 1 D, 2
fn main() {
    let f_value = got_number("Enter the first value: ");


    println!("Write your operation type between");
    println!("| + | - | * | / |");
    let input_operator = set_operation();

    
    let s_value = got_number("Enter the second value: ");
    
    match set_calculate(input_operator, f_value, s_value) {
        Ok(operation) => {
            println!("Calculating...");
            let result = operation.calculate();
            println!("Result: {}", result);
        }
        Err(e) => {
            println!("Error: {}", e);
        } 
    }
}



impl Operation {
    fn calculate(&self) -> f64 {
        match  *self  {

        Operation::Add(x, y) => 
            x + y
        ,
        Operation::Substract(x, y) => 
            x - y
        ,
        Operation::Multiply(x, y) => 
            x * y
        ,
        Operation::Divide(x, y) => 
          if y != 0.0{
            x / y
          } else {
            panic!("You can't divide by 0");
          }
     }
    } 
}


fn got_number (prompt: &str) -> f64 {
    loop{
        let mut input = String::new();
        println!("{}", prompt);
        io::stdin().read_line(&mut input).unwrap();

        // Try to parse the input as a floating point number (f64)
        match input.trim().parse::<f64>() {
            Ok(num) => return num, // Return the valid number
            Err(_) => println!("Invalid input. Please enter a valid number."), // Ask again if invalid
        }
    }
}

fn set_operation() -> String{
    loop{
        let mut input = String::new();
        println!("Enter an operator ( + | - | * | / ):");
        io::stdin().read_line(&mut input).unwrap();
        let operation =  input.trim().to_string();
        
        // Validate operator
        match operation.as_str() {
            "+" | "-" | "*" | "/" => return operation, // Return the valid operator
            _ => println!("Invalid operator. Please enter a valid operator ( + | - | * | / )."),
        }
    }
}



fn set_calculate(op: String, x: f64, y: f64) ->  Result<Operation, String> {
    match op.as_str() {
       "+" => Ok(Operation::Add(x, y)),
       "-" => Ok(Operation::Substract(x, y)),
       "*" => Ok(Operation::Multiply(x, y)),
       "/" => Ok(Operation::Divide(x, y)),
       _ => Err(format!("Invalid operator: {}", op)),

    }
}


#[cfg(test)]
 mod tests {
     use super::*;

     #[test]
     fn test_addition() {
         let operation = Operation::Add(2.0, 3.0);
         let result = operation.calculate();
         assert_eq!(result, 5.0);
     } 
     
     #[test]
     fn test_subtraction() {
         let operation = Operation::Substract(5.0, 3.0);
         let result = operation.calculate();
         assert_eq!(result, 2.0);
     }

     #[test]
     fn test_multiplication() {
         let operation = Operation::Multiply(4.0, 2.0);
         let result = operation.calculate();
         assert_eq!(result, 8.0);
     }

     #[test]
     fn test_division() {
         let operation = Operation::Divide(10.0, 2.0);
         let result = operation.calculate();
         assert_eq!(result, 5.0);
     }

     #[test]
    #[should_panic(expected = "You can't divide by 0")]
    fn test_division_by_zero() {
        let operation = Operation::Divide(10.0, 0.0);
        operation.calculate(); // This should panic
    } 
 
}