fn main() {
    // if表达式
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    if number != 0{
        println!("number is not zero");
    }

    // 多个if else
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // if 表达式用在let语句中
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("number is: {}", number);

    // if 分支值类型必须一致
    // let number = if condition { 5 } else { "six" };

}
