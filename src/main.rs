use std::any::type_name;
use std::io;


  



fn main() {
    println!("To convert Fahrenheit to Celsius enter 1 ");
    println!("To convert Celsius to Fahrenheit enter 2 ");
    let mut convertion_choice = String::new();
    io::stdin().read_line(& mut convertion_choice);
    let choice:i32 = convertion_choice.trim().parse().expect("something weird happened");
    
    if choice == 1 {
        println!("Enter Fahrenheit to convert to Celsius");
        let mut fahrenheit_input = String::new();
        io::stdin().read_line(& mut fahrenheit_input);
        let fahrenheit_num : i32 = fahrenheit_input.trim().parse().expect("can't run fahrenheit to int");
        let mut ftc = fahrenheit_to_celsius(fahrenheit_num);
        println!("{} fahrenheit is celsius  {} ",fahrenheit_num,ftc)

    } else if  choice == 2 {
        println!("Enter Celsius to convert Fahrenheit");
        let mut celsius_input = String::new();
        io::stdin().read_line(& mut celsius_input);
        let celsius_num : i32 = celsius_input.trim().parse().expect("can't turn celsius string into int");
        let ctf = celsius_to_fahrenheit(celsius_num);
        println!("{} celsius is fahrenheit  {} ",celsius_num,ctf)
    }

}



fn fahrenheit_to_celsius(x:i32) -> f64 {
    (((x as f64 -32 as f64) * 5 as f64) / 9 as f64)
}

fn celsius_to_fahrenheit(x:i32) -> f64 {
    ((x as f64 * 9 as f64) / 5 as f64) + 32 as f64
}