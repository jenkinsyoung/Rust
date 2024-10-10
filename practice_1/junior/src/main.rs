fn divide_numbers(a: f64, b: f64) -> Result<f64, &'static str> {
    if b == 0.0 {
        return Err("Division by zero is not allowed.");
    } else if a < 0.0 || b < 0.0{
        return Err("Division of negative numbers is not allowed. Please input only positive numbers.");
    }
    Ok(a / b)
}

fn main() {
    let x = 10.0;
    let y = 0.0;

    let result = divide_numbers(x, y);
    
    match result {
        Ok(value) => println!("Result: {}", value),
        Err(e) => println!("An error occurred: {} You can't do this: {:?} / {:?}", e, x, y),
    }

    let b = 5.0;
    let result_2 = divide_numbers(x, b);
    match result_2 {
        Ok(value) => println!("Second result: {}", value),
        Err(e) => println!("An error occurred: {} You can't do this: {:?} / {:?}", e, x, b),
    }

    let b = -5.0;
    let result_3 = divide_numbers(x, b);
    match result_3 {
        Ok(value) => println!("Second result: {}", value),
        Err(e) => println!("An error occurred: {} You can't do this: {:?} / {:?}", e, x, b),
    }

    let b = 5.0;
    let x = -10.0;
    let result_3 = divide_numbers(x, b);
    match result_3 {
        Ok(value) => println!("Second result: {}", value),
        Err(e) => println!("An error occurred: {} You can't do this: {:?} / {:?}", e, x, b),
    }

    let b = -5.0;
    let x = -10.0;
    let result_3 = divide_numbers(x, b);
    match result_3 {
        Ok(value) => println!("Second result: {}", value),
        Err(e) => println!("An error occurred: {} You can't do this: {:?} / {:?}", e, x, b),
    }
}
