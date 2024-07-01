use std::io;

fn main() {
    loop {
        println!("Temperature Converter");
        println!("1. Convert Celsius to Fahrenheit");
        println!("2. Convert Fahrenheit to Celsius");
        println!("3. Exit");

        let choice = get_input("Please choose an option (1/2/3): ");

        match choice.trim() {
            "1" => {
                let celsius = get_input("Enter temperature in Celsius: ");
                match celsius.trim().parse::<f64>() {
                    Ok(c) => {
                        let fahrenheit = celsius_to_fahrenheit(c);
                        println!("{:.2} Celsius is {:.2} Fahrenheit\n", c, fahrenheit);
                    }
                    Err(_) => println!("Please enter a valid number\n"),
                }
            }
            "2" => {
                let fahrenheit = get_input("Enter temperature in Fahrenheit: ");
                match fahrenheit.trim().parse::<f64>() {
                    Ok(f) => {
                        let celsius = fahrenheit_to_celsius(f);
                        println!("{:.2} Fahrenheit is {:.2} Celsius\n", f, celsius);
                    }
                    Err(_) => println!("Please enter a valid number\n"),
                }
            }
            "3" => {
                println!("Exiting...");
                break;
            }
            _ => println!("Invalid option. Please try again.\n"),
        }
    }
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    (c * 9.0 / 5.0) + 32.0
}

fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

fn get_input(prompt: &str) -> String {
    let mut input = String::new();
    println!("{}", prompt);
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input
}
