use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Usage: {} [int] [unit(:c|f)]", args[0]);
        return;
    };

    let temp: f32 = args[1].parse().unwrap();
    let unit = &args[2];
    let result: f32;
    if unit == "c" {
        result = (temp * 9.0 / 5.0) + 32.0;
        println!("{}°C = {}°F", temp, result);
    } else if unit == "f" {
        result = (temp - 32.0) * 5.0 / 9.0;
        println!("{}°F = {}°C", temp, result);
    } else {
        println!("Invalid unit. Use 'c' for Celsius or 'f' for Fahrenheit.");
        return;
    }
    return;
}
