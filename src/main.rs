use std::io;

fn main() {
    loop {
        println!("To convert Fahrenheit to Celsius enter 1");
        println!("To convert Celsius to Fahrenheit enter 2");
        println!("Enter your choice:");

        let choice = read_input().trim().parse::<i32>();
        match choice {
            Ok(1) => {
                println!("Enter Fahrenheit to convert to Celsius:");
                let fahrenheit = read_input().trim().parse::<f64>().expect("Invalid Fahrenheit input");
                let celsius = fahrenheit_to_celsius(fahrenheit);
                println!("{} Fahrenheit is {:.2} Celsius", fahrenheit, celsius);
            },
            Ok(2) => {
                println!("Enter Celsius to convert to Fahrenheit:");
                let celsius = read_input().trim().parse::<f64>().expect("Invalid Celsius input");
                let fahrenheit = celsius_to_fahrenheit(celsius);
                println!("{} Celsius is {:.2} Fahrenheit", celsius, fahrenheit);
            },
            _ => {
                println!("Invalid choice! Please enter 1 or 2.");
                continue;  
            },
        }
        break;  
    }
}

fn read_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input
}

fn fahrenheit_to_celsius(x: f64) -> f64 {
    (x - 32.0) * 5.0 / 9.0
}

fn celsius_to_fahrenheit(x: f64) -> f64 {
    (x * 9.0 / 5.0) + 32.0
}
