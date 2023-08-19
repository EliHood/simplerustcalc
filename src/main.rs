use std::io;

fn add(x1: i32, x2: i32) -> i32 {
    let total = x1 + x2;
    println!("Total is: {}", total);
    return total;
}
fn subtract(x1: i32, x2: i32) -> i32 {
    let total = x1 - x2;
    println!("Total is: {}", total);
    return total;
}
fn multiply(x1: i32, x2: i32) -> i32 {
    let total = x1 * x2;
    println!("Total is: {}", total);
    return total;
}
fn divide(x1: i32, x2: i32) -> i32 {
    let total = x1 / x2;
    println!("Total is: {}", total);
    return total;
}

fn calc(operation: fn(num1: i32, num2: i32) -> i32) -> i32 {
    let mut input_number = String::new();
    println!("Enter Number:");
    io::stdin()
        .read_line(&mut input_number)
        .expect("Not a valid string");

    println!("Enter another number:");
    let mut input_number2 = String::new();
    io::stdin()
        .read_line(&mut input_number2)
        .expect("Not a valid string");

    let parsed_numb1: i32 = input_number.trim().parse().unwrap();
    let parsed_numb2: i32 = input_number2.trim().parse().unwrap();
    return operation(parsed_numb1, parsed_numb2);
}

#[derive(Debug)]
pub enum OurCalc {
    Add,
    Subtract,
    Multiply,
    Divide,
}
#[derive(Debug)]
struct InputError;
impl OurCalc {
    fn from_str(s: &str) -> Result<OurCalc, InputError> {
        match s {
            "add" => Ok(OurCalc::Add),
            "substract" => Ok(OurCalc::Subtract),
            "multiply" => Ok(OurCalc::Multiply),
            "divide" => Ok(OurCalc::Divide),
            _ => Err(InputError),
        }
    }
}

fn match_calc(our_calc: OurCalc) -> i32 {
    match our_calc {
        OurCalc::Add => calc(add),
        OurCalc::Subtract => calc(subtract),
        OurCalc::Multiply => calc(multiply),
        OurCalc::Divide => calc(divide),
    }
}

fn main() {
    let mut input_string = String::new();
    println!("Enter either add, substract, multiply, or divide:");
    io::stdin().read_line(&mut input_string).unwrap();
    let calc = OurCalc::from_str(&input_string.trim()).unwrap();
    match_calc(calc);
}
