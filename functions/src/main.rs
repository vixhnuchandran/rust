fn main() {
    let add_result = add(5,3);
    println!("Addition result is : {}", add_result);

    let sub_result = sub(20,17);
    println!("Subtraction result is : {}", sub_result);

    let mul_result = mul(5,2);
    println!("Multiplication result is : {}", mul_result);

    let div_result = div(10,2);
    println!("Division result is : {}", div_result);


}


fn add (num1: i32, num2 :i32) -> i32 {
    num1 + num2
}


fn sub (num1: i32, num2 :i32) -> i32 {
    num1 - num2
}


fn mul (num1: i32, num2 :i32) -> i32 {
    num1 * num2
}


fn div (num1: i32, num2 :i32) -> i32 {
    num1 / num2
}