fn plus_one(x: i32) -> i32 {
    x + 1
}

fn return_number(condition:bool) -> i32 {
    let x = if condition {
        6
    } else {
        7
    };

    x
}

fn main() {
    let x:f32 = 2.4;
    let f:bool = true;
    let tup:(i32, f64, u8) = (500, 6.4, 1);
    // appending _ disable unused variable warning
    let (_x, _y, _z) = tup;

    println!("The value of y is: {}", _y);
    println!("The value of y is: {}", tup.1);

    let a = [1, 2, 3, 4, 5];
    // 5 element array filled all value with 3
    let a1 = [3; 5];
    println!("element: {}, {}", a[4], a1[3]);

    let ret = plus_one(5);

    println!("The value of ret is: {}", ret);

    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let ret1 = return_number(true);
    let ret2 = return_number(false);

    println!("ret1: {} and ret2: {}", ret1, ret2);

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);

    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");

    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }

    for element in a.iter() {
        println!("the value is: {}", element);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
