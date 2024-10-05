use std::io;
use std::process::Command;
// use std::result;


enum Operation {
    Add (f64, f64 ),
    Substract (f64, f64 ),
    Multiply (f64, f64 ),
    Divide (f64,f64 ),
}

// Steps 1 D, 2
fn main() {
    println!("Write your operation type between");
    println!("| + | - | * | / |");
    let input_operator = set_operation();

    let f_value = got_number("Enter the first value: ");

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
          if(y != 0.0){
            x / y
          } else {
            panic!("You can't divide by 0");
          }
     }
    } 
}


fn got_number (prompt: &str) -> f64 {
    let mut input = String::new();
    println!("{}", prompt);
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
// fn calculate(op: Operation ) -> f64 {;
//     match op {
//         Operation::Add(x, y) => {
//             x + y
//         },
//         Operation::Substract(x, y) => {
//             x - y
//         },
//         Operation::Multiply(x, y) => {
//             x * y
//         },
//         Operation::Divide(x, y) => {
//           if(y != 0.0){
//             x / y
//           } else {
//             panic!("You can't divide by 0");
//           }
//         }
//     }
// }

// Testing

#[cfg(test)]
 mod tests {
     use super::*;

     #[test]
    fn test_set_calculate_add(){
        let op = "+".to_string();
        let result = set_calculate(op, 3.0, 4.0);
        assert_eq!(result, Ok(Operation::Add(3.0, 4.0)));
    } 

    #[test]
    fn test_invalid_operator() {
        let op = "%".to_string();
        let result = set_calculate(op, 10.0, 2.0);
        assert_eq!(result, Err("Invalid operator: %".to_string()));
    }
    


}