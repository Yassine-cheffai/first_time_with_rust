use std::io;

fn add(x: &i32, y: &i32) -> i32 {
    x + y
}

fn substract(x: &i32, y: &i32) -> i32 {
    x - y
}

fn multiply(x: &i32, y: &i32) -> i32 {
    x * y
}

fn divide(x: &i32, y: &i32) -> i32 {
    // FIXME
    if y > &0 {
        x / y
    } else {
        0
    }
}

fn main() {
    let mut input = String::new();
    let mut x = String::new();
    let mut y = String::new();

    println!("Type the value of x: ");
    match io::stdin().read_line(&mut x) {
        Ok(_) => {}
        Err(_) => {
            println!("Something went wrong");
        }
    }

    println!("Type the value of y: ");
    match io::stdin().read_line(&mut y) {
        Ok(_) => {}
        Err(_) => {
            println!("Something went wrong");
        }
    }

    println!("Choose your operation type: ");
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            let x = x.trim().parse::<i32>().unwrap();
            let y = y.trim().parse::<i32>().unwrap();
            let signe = input.trim();
            let mut result: i32 = 0;

            if signe == "+" {
                result = add(&x, &y);
            }

            if signe == "-" {
                result = substract(&x, &y);
            }

            if signe == "*" {
                result = multiply(&x, &y);
            }

            if signe == "/" {
                result = divide(&x, &y);
            }
            println!("Result = {}", result);
        }
        Err(e) => println!("Oops! Something went wrong: {}", e),
    }
}
