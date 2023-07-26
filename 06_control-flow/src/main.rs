fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // Using if in a let Statement
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");

    let mut x: i32 = 0;

    // Repetition with Loops
    loop {
        x += 1;
        println!("again!");
        if x > 7 {
            break;
        }
    }

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");

    // Loop Labels to Disambiguate Between Multiple Loops
    let mut count = 0;

    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
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
    println!("End count = {count}");

    // Conditional Loops with while
    let mut my_number = 3;

    while my_number != 0 {
        println!("{my_number}!");

        my_number -= 1;
    }

    println!("LIFTOFF!!!");

    // Looping Through a Collection with for
    let array: [i32; 5] = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", array[index]);

        index += 1;
    }

    // For loop
    for element in array {
        println!("the value is: {element}");
    }

    // For with range instead of while loop
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
