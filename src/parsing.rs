mod primary_operations;

//error handling

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
//enums
enum ErrorType {
    Parser,
    Processor,
}

#[derive(Debug)]
pub struct Error {
    kind: ErrorType,
}

impl Error {
    fn could_not_parse_function() -> Self {
        Self {
            kind: ErrorType::Parser,
        }
    }
    fn operation_not_found() -> Self {
        Self {
            kind: ErrorType::Processor,
        }
    }
}


pub fn call_operations(input: String) -> String{

    let mut result = "Er".to_string();//this is the result that is piped into js
    //check the operation
    if input.find('+') != None {
        println!("Addition");

        let vec = match parse_input(&input, '+') {
            Ok(input) => input,
            Err(_) => {//TODO: Implement more comprehensive error handling and error messaging
                println!("Error occurred when reading parsed value, 
                this should be changed into an actual error");
                Vec::new()
            }
        };

        let a: f32 = vec[0].parse().unwrap();
        let b: f32 = vec[1].parse().unwrap();

        result = primary_operations::add(a, b).to_string();

        println!("Result: {}", result)
        
    }

    else if input.find('-') != None {
        println!("Subtraction");
        let vec = match parse_input(&input, '-') {
            Ok(input) => input,
            Err(_) => {//TODO: Implement more comprehensive error handling and error messaging
                println!("Error occurred when reading parsed value, 
                this should be changed into an actual error");
                Vec::new()
            }
        };

        let a: f32 = vec[0].parse().unwrap();
        let b: f32 = vec[1].parse().unwrap();
        result = primary_operations::sub(a, b).to_string();
        println!("Result: {}", result);
    }

    else if input.find('*') != None {
        println!("Multiplication");
        let vec = match parse_input(&input, '*') {
            Ok(input) => input,
            Err(_) => {//TODO: Implement more comprehensive error handling and error messaging
                println!("Error occurred when reading parsed value, 
                this should be changed into an actual error");
                Vec::new()
            }
        };

        let a: f32 = vec[0].parse().unwrap();
        let b: f32 = vec[1].parse().unwrap();
        result = primary_operations::mult(a, b).to_string();
        println!("Result: {}", result);
    }

    else if input.find('/') != None {
        println!("Division");
        let vec = match parse_input(&input, '/') {
            Ok(input) => input,
            Err(_) => {//TODO: Implement more comprehensive error handling and error messaging
                println!("Error occurred when reading parsed value, 
                this should be changed into an actual error");
                Vec::new()
            }
        };

        let a: f32 = vec[0].parse().unwrap();
        let b: f32 = vec[1].parse().unwrap();
        result = primary_operations::div(a, b).to_string();
        println!("Result: {}",result);
    }
    result
}

//TODO:This should be more complicated and should be able to parse a whole operation
//TODO:The current version is too simple and static as it only accounts for a single operation
//TODO:This should be divided into multiple functions that each handle a different operation.
//TODO:It should also have a start function that points to the correct operation
fn parse_input(input:&String, sign:char) -> Result<Vec<String>>{
    let vec = input.split(sign).collect::<Vec<&str>>();
    let right_num = &vec[1].split('\r').map(str::to_string).collect::<Vec<String>>()[0];
    let final_vec = vec![vec[0].to_string(), right_num.to_string()];
    Ok(final_vec)
}
//using the shunting yard algorithm by dijkstra
    //1+(2*3-1)-2 becomes 123*1-+2-   Stack:
    //We need a stack that holds temporary operations
    //we also need conditions for priority of different operations and parenthesis
    //TODO:This is not done as we need to look into precedence and associativity of different operators and add them
    //TODO:Add negation(-5) and power(5^5)
    /*
    Table of operation priority
    op    prio
    {^}   1
    {-}   2
    {* /} 3
    {+ -} 4

    */

///gets a infix string and converts it to a postfix string vector
//TODO remove all unwrap in the program to not encounter panics
pub fn to_postfix(infix: &String) -> Vec<String>{
    return rust_prefix_to_postfix::reverse_polish_parsing(infix).unwrap();
}


/*
TODO parse the vector from right to left to find the first operator and then match that operator and the two
     vales before that operator
*/
///parses postfix and calls the appropriate operation for each index of the postfix vector
///this function consumes the sent postfix string vector
pub fn execute_postfix(postfix: Vec<String>) -> Result<String>{
    let mut cloned_postfix: Vec<String> = postfix.clone();
    let mut number_stack: Vec<String> = vec![];
    let operations: Vec<String> = vec!["+".to_string(), "-".to_string(), "/".to_string(),
         "*".to_string(), "%".to_string(), "^".to_string()];
    let functions: Vec<String> = vec!["sqrt".to_string(), "max".to_string(), "min".to_string()];

    let mut first_element_str: String = "".to_string();
    let mut second_element_str: String = "".to_string();
         
    for (index, element) in postfix.iter().enumerate(){
        if operations.contains(element){
            //println!("found an operator {:?} {:?}", element, number_stack);
            //we have found an operator
            //we need to read values from the stack and pop them
            
            let mut first_element: f32 = 0.0;
            let mut second_element: f32 = 0.0;

            if number_stack.len() > 1 {
                second_element_str = number_stack.pop().unwrap();
                first_element_str = number_stack.pop().unwrap();
            }
            else if number_stack.len() == 1 {
                first_element_str = number_stack.pop().unwrap();
            }
            if !functions.contains(&first_element_str){
                first_element = first_element_str.parse::<f32>().unwrap();
            }
            if !functions.contains(&second_element_str){
                second_element =second_element_str.parse::<f32>().unwrap();
            }
            let mut local_total: f32 = 0.0;
            if functions.contains(&first_element_str){
                if first_element_str.to_string() == "sqrt"{
                    local_total = primary_operations::sqrt(second_element);
                }
                let prev_element = number_stack.last().unwrap().parse::<f32>().unwrap();
                local_total = match element.as_str() {
                    "+" => {primary_operations::add(prev_element, local_total)},
                    "-" => {primary_operations::sub(prev_element, local_total)},
                    "/" => {primary_operations::div(prev_element, local_total)},
                    "*" => {primary_operations::mult(prev_element, local_total)},
                    "%" => {0.0},
                    "^" => {primary_operations::pow(prev_element, local_total)},
                    //"sqrt" => {primary_operations::sqrt(first_element)},
                    //max and min are not supported yet
                    //"max" => {primary_operations::max(*first_element, *second_element)},
                    //"min" => {},
                    //needs better error handling
                    _ => {0.0},
                };
            }

            else{
                local_total = match element.as_str() {
                    "+" => {primary_operations::add(first_element, second_element)},
                    "-" => {primary_operations::sub(first_element, second_element)},
                    "/" => {primary_operations::div(first_element, second_element)},
                    "*" => {primary_operations::mult(first_element, second_element)},
                    "%" => {0.0},
                    "^" => {primary_operations::pow(first_element, second_element)},
                    //"sqrt" => {primary_operations::sqrt(first_element)},
                    //max and min are not supported yet
                    //"max" => {primary_operations::max(*first_element, *second_element)},
                    //"min" => {},
                    //needs better error handling
                    _ => {0.0},
                };
            }
            number_stack.push(local_total.to_string());
            if index == postfix.len()-1 {
                return Ok(local_total.to_string());
            }
        }
        else{
            //it is a number
            //println!("found a number {:?} {:?}", element, number_stack);
            let taken_element = std::mem::take(&mut cloned_postfix[index]);
            number_stack.push(taken_element);
        }
    }

    Ok("Erorr".to_string())
}