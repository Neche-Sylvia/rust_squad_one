use std::io;
fn main() {
    let mut input = String::new();

   println!("Enter First number");
   io::stdin().read_line(&mut input).expect("Failed to read line");
   let num1: f64 = input.trim().parse().expect("Please enter a valid number");

   input.clear();

   println!("Enter Second number");
   io::stdin().read_line(&mut input).expect("Failed to read line");
   let num2: f64 = input.trim().parse().expect("Please enter a valid number");

   input.clear();

   println!("Enter Operator");
   io::stdin().read_line(&mut input).expect("Failed to read line");
   let operator = input.trim().to_string();

   let mut Calc = if operator == "+" {
    num1 + num2
   }
   else if operator == "-" {
    if num1 > num2{
        num1 - num2
    }
    else {
        num2 - num1
    }
   }
   else if operator == "*" {
    num1 * num2
   }
   else if operator == "/" {
    if num1  > num2 {
        num1  / num2 
    }
    else {
        num2  / num1 
    }
   }

   else {
    println!("Wrong operator");
    0.0
   };

   println!("The solution is: {}", Calc);
}