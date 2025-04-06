fn main() {
    // 字符串转数字
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("Guess is {}", guess);

    // 类型转换时必须要加类型
    // let guess2 = "42".parse().expect("Not a number!");

    // 非数字型字符串转换时会报错，expect会返回错误信息
    // let guess3 :u32 = "again".parse().expect("Not a number!");

    // 数值型算数运算
    // +
    let sum = 5 + 10;
    // -
    let difference = 95.5 - 4.3;
    // *
    let product = 4 * 30;
    // /
    let quotient = 56.7 / 3.2;
    let truncated = -5 / 3;

    // %
    let remainder = 43 % 5;
    println!("Sum: {}, Difference: {}, Product: {}, Quotient: {}, Truncated: {}, Remainder: {}", sum, difference, product, quotient, truncated, remainder);

    // bool类型
    let t = true;
    let f: bool = false;
    println!("t: {}, f: {}", t, f);

    // char 类型
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
    println!("c: {}, z: {}, heart_eyed_cat: {}", c, z, heart_eyed_cat);

    // tuple 类型
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("x: {}, y: {}, z: {}", x, y, z);

    // tuple使用索引取值
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
    println!("five_hundred: {}, six_point_four: {}, one: {}", five_hundred, six_point_four, one);

    // 数组类型
    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];
    println!("first: {}, second: {}", first, second);

    // 数组类型显式类型注解
    let b: [i32; 5] = [1, 2, 3, 4, 5];
    let third = b[2];
    println!("third: {}", third);

    // 数组初始化填充
    let a1 = [3; 5];
    // 等同于
    // let a2 = [3, 3, 3, 3, 3];
    println!("a1: {:?}", a1);

}


