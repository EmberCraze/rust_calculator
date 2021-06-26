mod primary_operations;

use std::io::*;

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
fn parse_input_shunting_yard(input:&String){// -> Result<Vec<String>>
    let math_ops_prio_1 = vec!["+".to_string(), "-".to_string()];
    let math_ops_prio_2 = vec!["*".to_string(), "/".to_string(), "%".to_string()];
    let mut symbol_stack: Vec<String> = vec![];
    let mut output: Vec<String> = vec![];
    let mut last_is_number = false;
    let mut next_output_value = "".to_string();


    for i in input.chars() {
        if i.is_digit(10) {
            //i must be a digit
            println!("{} is a digit", i);
            if last_is_number == true {
                next_output_value = next_output_value + &i.to_string();//format!("{}{}", next_output_value, i);//this concatenates i to the variable
            }
            else{
                next_output_value = i.to_string();
            }
            last_is_number = true;
            if i == input.chars().last().unwrap(){
                output.push(next_output_value.to_string());
                last_is_number = false;
            }
        }
        else if math_ops_prio_2.contains(&i.to_string()){
            //i must be an operator of prio 2
            println!("{} is a prio 2 operator", i);
            if last_is_number == true{
                output.push(next_output_value.to_string());
                last_is_number = false;
            }
            loop{
                //if the last value in the stack is of lower prio or if the stack i empty
                //push the value to the stack
                //if the last value is not prio 2 and is not an end parenthesis add the prio 2 operand
                if !math_ops_prio_2.contains(&symbol_stack[symbol_stack.len()-1]) 
                                && symbol_stack[symbol_stack.len()-1] != ")"{
                    symbol_stack.push(i.to_string());
                    break;
                }
                else{
                    //pop the last symbol and add it to the output
                    output.push(symbol_stack[symbol_stack.len()-1].clone());
                    symbol_stack.pop();
                }
            }
        }
        else if math_ops_prio_1.contains(&i.to_string()){
            //i must be and operator of prio 1
            println!("{} is a prio 1 operator", i);
            if last_is_number == true{
                output.push(next_output_value.to_string());
                last_is_number = false;
            }
            loop{
                //if the last value in the stack is a start parenthesis or 
                //if the stack is empty push the prio 1 operator to the stack
                //else pop all symbols that are higher priority to the output
                if symbol_stack.len() == 0 || symbol_stack[symbol_stack.len()-1] == "(" {
                    symbol_stack.push(i.to_string());
                    break;
                }
                else{
                    output.push(symbol_stack[symbol_stack.len()-1].clone());
                    symbol_stack.pop();
                }
            }
        }
        else if i == '('{
            //i is a start parenthesis
            println!("{} is a start parenthesis", i);
            if last_is_number == true{
                output.push(next_output_value.to_string());
                last_is_number = false;
            }
            //we always place it on the stack
            symbol_stack.push(i.to_string());
        }
        else if i == ')'{
            //i is a end parenthesis
            println!("{} is an end parenthesis", i);
            if last_is_number == true{
                output.push(next_output_value.to_string());
                last_is_number = false;
            }
            loop{
                //pop symbols to the output until it reaches an start parenthesis
                if symbol_stack[symbol_stack.len()-1] != "("{
                    output.push(symbol_stack[symbol_stack.len()-1].clone());
                    symbol_stack.pop();
                }
                else{
                    symbol_stack.pop();
                    break;
                }
            }
        }
    }
    //emptying the stack
    for j in symbol_stack.clone(){
        output.push(symbol_stack[symbol_stack.len()-1].clone());
        symbol_stack.pop();
    }
    println!("{:?}", symbol_stack);
    println!("{:?}", output);
}