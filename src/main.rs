// my hello world program
fn main() {
    /* multiple
    line
    comment */
    println!("Hello, world!");

    /*let mut x: u8 = 255;
    x = x + 1;
    attempt to add with overflow*/
    
    /* changing mutable value
    println!("x is {}", x); 
    x = 20;
    println!("x is {}", x); */

    /*let x: f32 = 10.123456789123456789;
    println!("x is {}", x); */

    /*let a = 10.0;
    let b = 3.0;
    let c = a / b;
    print!("c is {0:08.3}\na is {1}\nonce again, c is {0}", c, a);*/
    
    /*let mut value = 0b1111_0101u8;
    println!("Value is {}", value);
    println!("Value is {:08b}", value);

    value = !value;
    println!("Value is {:08b}", value);

    value = value & 0b1111_0111;
    println!("Value is {:08b}", value);
    println!("bit 6 is {}", value & 0b0100_0000);

    value = value | 0b0100_0000;
    println!("Value is {:08b}", value);

    value = value ^ 0b0101_0101;
    println!("Value is {:08b}", value);

    value = value  << 4;
    println!("Value is {:08b}", value);

    value = value >> 2;
    println!("Value is {:08b}", value);*/

    /*let a = true;
    let b = false;
    println!("a is {} and b is {}", a, b);
    println!("NOT a is {}", !a);
    println!("a AND b is {}", a & b);
    println!("a or b is {}", a | b);
    println!("a XOR b is {}", a ^ b);

    let c = (a ^ b) && panic!();
    println!("c is {}", c);*/

    /*let a = true;
    let b = false;
    println!("a is {} and b is {}", a, b);
    println!("a EQUAL TO b is {}", a == b);
    println!("a NOT EQUAL TO b is {}", a != b);
    println!("a GREATER THAN b is {}", a > b);
    println!("a GREATER THAN OR EQUAL TO b is {}", a >= b);
    println!("a LESS THAN b is {}", a < b);
    println!("a LESS THAN OR EQUAL TO b is {}", a <= b);*/

    /*let letter = 'a';
    let number = '1';
    let finger = '\u{261D}';
    println!("{}\n{}\n{}", letter, number, finger);*/

    //Challenge
    /*let a = 13;
    let b = 2.3;
    let c: f32 = 120.0;

    let average = (a as f64 + b + c as f64) / 3.0;

    assert_eq!(average, 45.1);
    println!("Test passed!");*/

    /*let mut letters = ['a', 'b', 'c'];
    letters[0] = 'x';
    let first_letter = letters[0];
    println!("first letter is {}", first_letter);

    let numbers: [i32; 5];
    numbers = [0; 5];
    let index = numbers.len();
    println!("last number is {}", numbers[index - 1]);*/

    /*let parking_lot = [[1, 2, 3], 
                                      [4, 5, 6]];

    let number = parking_lot[1][2];
    println!("number is {}", number);

    let garage: [[[0; 100]; 20]; 5];*/

    /*let mut stuff: (u8, f32, char) = (10, 3.14, 'x');
    stuff.0 += 3;
    let first_item = stuff.0;
    println!("first item is {}", first_item);

    let (a, b, c) = stuff;
    println!("b is {}", b);*/

    /*say_hello();
    say_hello();
    let x = 1;
    let y = 2;
    say_the_sum(x, y);*/

    /*let result = square(13);
    println!("result is {:?}", result);*/

    /*let celsius_temp = 23.0;
    let fahrenheit_temp = celsius_to_fahrenheit(celsius_temp);

    assert_eq!(fahrenheit_temp, 73.4);
    println!("Test passed!");*/

    /*let x = 4;

    if x + 1 != 3{
        println!("x is NOT 3!");
    }*/

    /*let x = 3;
    let y = 5;

    if x > y {
        println!("x is greater than y");
    } else {
        if x < y{
            println!("x is less than y");
        } else {
            println!("x is equal to y");
        }
    }

    if x > y {
        println!("x is greater than y");
    } else if x < y{
            println!("x is less than y");
    } else {
            println!("x is equal to y");
    }*/

    /*let make_x_odd = true;
    let x = if make_x_odd {1} else {2};

    /*if make_x_odd{
        x = 1;
    } else {
        x = 2;
    }*/

    println!("x is {}", x);*/

    /*let mut count = 0;

    let result =loop {
        if count == 10{
            break count * 10;
        }
        count += 1;
        println!("count is {}", count);
    };
    println!("After the loop!");
    println!("result is {}", result);*/

    /*let mut count = 0;
    let letters = ['a', 'b', 'c'];

    while count < letters.len(){
        println!("count is {}", letters[count]);
        count += 1;
    }*/

    /*let message = ['h', 'e', 'l', 'l', 'o'];

    for (index, &item) in message.iter().enumerate(){
        println!("item {} is {}", index, item);
        if item == 'e'{
            break;
        }
    }

    for number in 0..5{
        println!("number is {}", number);
    }*/

    /*let mut matrix = [[1, 2, 3],
                            [4, 5, 6],
                            [7, 8, 9]];

    for row in matrix.iter_mut(){
        for num in row.iter_mut(){
            *num += 10;
            print!("{}\t", num);
        }
        println!();
    }*/

    //Challenge 3 - Max, min, mean
    let numbers = [1, 9, -2, 0, 23, 20, -7, 13, 37, 20, 56, -18, 20, 3];
    let mut max: i32;
    let mut min: i32;
    let mut mean: f64;

    /* CODE GOES HERE */
    max = numbers[0];
    min = numbers[0];
    mean = 0.0;

    for &num in numbers.iter(){
        if num > max{
            max = num;
        } else if num < min{
            min = num;
        }
        mean += num as f64;
    }

    mean /= numbers. len() as f64;

    assert_eq!(max, 56);
    assert_eq!(min, -18);
    assert_eq!(mean, 12.5);
    println!("Tests passed!");
    
}

/*fn say_hello(){
    println!("Hello!");
    say_a_number(13);
}

fn say_a_number(number: i32){
    println!("number is {}", number);
}

fn say_the_sum(a: u8, b: u8){
    let sum = a + b;
    println!("sum is {}", sum);
}*/

/*fn square(x: i32) -> (i32, i32){
    println!("squaring {}", x);
    return (x, x * x);
    println!("End of function"); //doesn't actually print
}*/

//Challenge 2 - Convert Temperature
/*fn celsius_to_fahrenheit(celsius: f64) -> f64{
    1.8 * celsius + 32.0
}*/
