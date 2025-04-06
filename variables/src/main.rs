fn main() {
    // 不可变
    let x = 5;
    println!("The value of x is: {}", x);

    // 可变
    let mut y = 5;
    println!("The value of y is: {}", y);
    y = 6;
    println!("The value of y is: {}", y);

    // 常量
    const THREE_HOURS_IN_SECONDS : u32 = 60 * 60 * 3;
    println!("{}", THREE_HOURS_IN_SECONDS);

    // Shadowing
    let a = 5;
    let a = a + 1;

    {
        let a = a * 2;
        println!("The value of a in the inner scope is: {}", a);
    }
    println!("The value of a is: {}", a);

    // Shadowing 不同的类型
    let spaces = "  ";
    let spaces = spaces.len();
    println!("The length of spaces is: {}", spaces);
}
