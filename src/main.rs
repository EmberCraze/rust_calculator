mod parsing;

fn main(){
    let postfix: Vec<String> = parsing::to_postfix(&".8+.7+7,7+((2^2)*3-1)-(2^2)+2-sqrt(5*7+3)".to_string());
    dbg!(parsing::execute_postfix(postfix).unwrap());
}