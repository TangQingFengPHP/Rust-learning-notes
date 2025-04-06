fn main() {
    another_function(5);
    print_labeled_measurement(10, 'C');

    // 块表达式
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {}", y);

    // 如果在块表达式里面添加了分号，则会变成statement而不是expression，不会返回值
    let z = {
        let a = 1;
        a + 1;
    };
    println!("The value of z is: {:?}", z);
    // 打印出来是()，因为z没有返回值

    // 函数最后一行表达式没有分号，即使没有return关键字，也会返回值
    // 调用five函数，并打印返回值
    let result = five();
    println!("The value of result is: {}", result);

    // 调用plus_one函数，并打印返回值
    let result = plus_one(4);
    println!("The value of result is: {}", result);
}

fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}