fn main() {
   let mut ftc = fahrenheit_to_celsius(30);
   println!("fahrenheit to celsius is {} ",ftc)

//   let ctf = celsius_to_fahrenheit(-1);
//   println!("celsius to fahrenheit is {} ",ctf)
}

fn fahrenheit_to_celsius(x:i32) -> f64 {
    (((x as f64 -32 as f64) * 5 as f64) / 9 as f64)
}

fn celsius_to_fahrenheit(x:i32) -> f64 {
    ((x as f64 * 9 as f64) / 5 as f64) + 32 as f64
}