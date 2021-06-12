use std::fmt;
enum Operator {
    Add,
    Multiply,
    Divide,
    Subtract,
    Exponent,
}
enum Token {
    Number(f64),
    Operator(Operator),
}
struct OperationVector {
    add: Option<fn(&Vec<Token>, &mut usize, &mut Vec<Token>)>,
    subtract: Option<fn(&Vec<Token>, &mut usize, &mut Vec<Token>)>,
    divide: Option<fn(&Vec<Token>, &mut usize, &mut Vec<Token>)>,
    multiply: Option<fn(&Vec<Token>, &mut usize, &mut Vec<Token>)>,
    exponent: Option<fn(&Vec<Token>, &mut usize, &mut Vec<Token>)>,
}
pub struct CalculationError {
    message: String
}
impl fmt::Display for CalculationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error with calculator: {}. Please Try Again!", self.message) // user-facing output
    }
}

// Implement std::fmt::Debug for AppError
impl fmt::Debug for CalculationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ file: {}, line: {}, message: {} }}", file!(), line!(), self.message) // programmer-facing output
    }
}
fn process_pass(result_array: &Vec<Token>, operations: OperationVector) -> Vec<Token> {
    let mut index: usize = 0;
    let mut new_array: Vec<Token> = Vec::new();
    while index < result_array.len() {
        match result_array[index] {
            Token::Number(number) => new_array.push(Token::Number(number)),
            Token::Operator(Operator::Add) => {
                match operations.add {
                    Some(func) => func(result_array, &mut index, &mut new_array),
                    None => new_array.push(Token::Operator(Operator::Add)),
                };
            }
            Token::Operator(Operator::Subtract) => {
                match operations.subtract {
                    Some(func) => func(result_array, &mut index, &mut new_array),
                    None => new_array.push(Token::Operator(Operator::Subtract)),
                };
            }
            Token::Operator(Operator::Multiply) => {
                match operations.multiply {
                    Some(func) => func(result_array, &mut index, &mut new_array),
                    None => new_array.push(Token::Operator(Operator::Multiply)),
                };
            }
            Token::Operator(Operator::Divide) => {
                match operations.divide {
                    Some(func) => func(result_array, &mut index, &mut new_array),
                    None => new_array.push(Token::Operator(Operator::Divide)),
                };
            }
            Token::Operator(Operator::Exponent) => {
                match operations.exponent {
                    Some(func) => func(result_array, &mut index, &mut new_array),
                    None => new_array.push(Token::Operator(Operator::Exponent)),
                };
            }
        }
        index += 1
    }
    return new_array;
}
// Resolve a string of calculations to the resulting number
pub fn calculate(to_calculate: &str) -> Result<f64,CalculationError> {
    let mut result_array: Vec<Token> = Vec::new();
    let mut number = String::from("");
    for character in to_calculate.chars() {
        if let '+' | '-' | '*' | '/' | '^' = character {
            result_array.push(Token::Number(number.clone().parse::<f64>().unwrap()));
            number = "".to_string();
        }
        match character {
            '*' => result_array.push(Token::Operator(Operator::Multiply)),
            '/' => result_array.push(Token::Operator(Operator::Divide)),
            '-' => result_array.push(Token::Operator(Operator::Subtract)),
            '+' => result_array.push(Token::Operator(Operator::Add)),
            '^' => result_array.push(Token::Operator(Operator::Exponent)),
            _ => {
                if character.is_ascii_digit() || character == '.' {
                    number.push(character)
                } else if !character.is_ascii_whitespace() {
                    return Err(CalculationError{
                        message: String::from(format!("Invalid character: {}",character))
                    })
                }
            }
        }
    }
    if number != "".to_string() {
        result_array.push(Token::Number(number.clone().parse::<f64>().unwrap()));
    }
    result_array = process_pass(
        &result_array,
        OperationVector {
            add: None,
            subtract: None,
            multiply: None,
            divide: None,
            exponent: Some(
                |result_array: &Vec<Token>, index: &mut usize, new_array: &mut Vec<Token>| {
                    if let Token::Number(number) = result_array[*index + (1 as usize)] {
                        if let Token::Number(last_number) = *new_array.last().unwrap() {
                            *new_array.last_mut().unwrap() =
                                Token::Number(f64::powf(last_number, number));
                            *index += 1 as usize;
                        }
                    }
                },
            ),
        },
    );
    result_array = process_pass(
        &result_array,
        OperationVector {
            add: None,
            subtract: None,
            multiply: Some(
                |result_array: &Vec<Token>, index: &mut usize, new_array: &mut Vec<Token>| {
                    if let Token::Number(number) = result_array[*index + (1 as usize)] {
                        if let Token::Number(last_number) = *new_array.last().unwrap() {
                            *new_array.last_mut().unwrap() = Token::Number(last_number * number);
                            *index += 1 as usize;
                        }
                    }
                },
            ),
            divide: Some(
                |result_array: &Vec<Token>, index: &mut usize, new_array: &mut Vec<Token>| {
                    if let Token::Number(number) = result_array[*index + (1 as usize)] {
                        if let Token::Number(last_number) = *new_array.last().unwrap() {
                            *new_array.last_mut().unwrap() = Token::Number(last_number / number);
                            *index += 1 as usize;
                        }
                    }
                },
            ),
            exponent: None,
        },
    );
    result_array = process_pass(
        &result_array,
        OperationVector {
            add: Some(
                |result_array: &Vec<Token>, index: &mut usize, new_array: &mut Vec<Token>| {
                    if let Token::Number(number) = result_array[*index + (1 as usize)] {
                        if let Token::Number(last_number) = *new_array.last().unwrap() {
                            *new_array.last_mut().unwrap() = Token::Number(last_number + number);
                            *index += 1 as usize;
                        }
                    }
                },
            ),
            subtract: Some(
                |result_array: &Vec<Token>, index: &mut usize, new_array: &mut Vec<Token>| {
                    if let Token::Number(number) = result_array[*index + (1 as usize)] {
                        if let Token::Number(last_number) = *new_array.last().unwrap() {
                            *new_array.last_mut().unwrap() = Token::Number(last_number - number);
                            *index += 1 as usize;
                        }
                    }
                },
            ),
            divide: None,
            multiply: None,
            exponent: None,
        },
    );
    if let Token::Number(value) = result_array[0] {
        return Ok(value);
    }
    return Ok(0.0);
}
#[cfg(test)] // Only compiles when running tests
mod tests { // Separates tests from code
    use crate::calculate;
    #[test]
    fn test_basic() {
        match calculate("2+2"){
            Ok(value)=>{assert_eq!(value,4.0)},
            Err(value)=>{assert!(false,value)}
        }
    }
    #[test]
    fn test_all() {
        match calculate("2+7*4^2-5"){
            Ok(value)=>{assert_eq!(value,109.0)},
            Err(value)=>{assert!(false,value)}
        }
    }
    #[test]
    fn test_decimal() {
        match calculate("2.1*2+5.35"){
            Ok(value)=>{assert_eq!(value,9.55)},
            Err(value)=>{assert!(false,value)}
        }
    }
    #[test]
    fn test_invalid() {
        match calculate("invalid"){
            Ok(value)=>{assert!(false,"Invalid character returned number: {}",value)},
            Err(value)=>{assert!(true,value)}
        }
    }
}