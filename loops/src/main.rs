fn main() {
    // loop 无尽循环
    // loop {
    //     println!("again!");
    // }
    // break 携带值
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is: {}", result);

    // loop 带有label
    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }

    println!("End count: {}", count);

    // while 循环
    let mut number = 3;

    while number != 0 {
        println!("number = {}", number);
        number -= 1;
    }

    println!("LIFTOFF!!!");

    // loop 循环数组
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < a.len() {
        println!("a[index] = {}", a[index]);
        index += 1;
    }

    // 使用 for in 循环数组
    for element in a {
        println!("element is {}", element);
    }

    // 使用 for in 反转遍历
    for revElement in (0..5).rev() {
        println!("revElement is {}", revElement);
    }

    // 使用 for in 反转遍历a数组
    for revElement in a.iter().rev() {
        println!("revElement is {}", revElement);
    }
}

