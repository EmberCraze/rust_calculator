//This file should contain primery functions that are + - * /



pub fn add(num1:f32, num2:f32) -> f32 {
    num1+num2
}

pub fn sub(num1:f32, num2:f32) -> f32 {
    num1-num2
}

pub fn mult(num1:f32, num2:f32) -> f32 {
    num1*num2
}

pub fn div(numerator:f32, denominator:f32) -> f32 {
    //num2 cannot be 0
    let mut div_result = 0.0;
    if denominator !=0.0 {
        div_result = numerator/denominator;
        //println!("Not 0");
       
    }
    else{
        println!("Division by 0");
    }
    div_result
}

pub fn pow(base: f32, power: f32) -> f32 {
    base.powf(power)
}

pub fn sqrt(value: f32) -> f32{
    if value > 0.0 {
        return pow(value, 0.5);
    }
    println!("Error negative numbers not supported");
    0.0
}

//sin cos tan are not complete as they need a radiance and degrees version and differentiation
pub fn sin_c(value: f32) -> f32{
    value.sin()
}

pub fn cos_c(value: f32) -> f32{
    value.cos()
}

pub fn tan_c(value: f32) -> f32{
    value.tan()
}

//TODO:add sin
//TODO:add cos
//TODO:add radians and degrees
