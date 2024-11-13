use std::io;

fn main() {
    loop {
        println!("Select an option.");
        println!("(1) - Convert Celsius to Fahrenheit");
        println!("(2) - Convert Fahrenheit to Celsius");

        let mut option = String::new();

        io::stdin()
            .read_line(&mut option)
            .expect("Failed to read line");

        let option: i32 = match option.trim().parse::<i32>() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if option > 0 && option < 3 {
            if option == 1 {
                loop {
                    println!("Enter temperature in Celsius");
                    let mut celsius = String::new();

                    io::stdin()
                        .read_line(&mut celsius)
                        .expect("Failed to read line");

                    let celsius: f64 = match celsius.trim().parse() {
                        Ok(num) => num,
                        Err(_) => continue,
                    };

                    let fahrenheit: f64;

                    fahrenheit = temperature_converter(celsius);

                    println!("Temperature in Fahrenheit: {fahrenheit}");
                    break;
                }
            } else if option == 2 {
                loop {
                    println!("Enter temperature in Fahrenheit");
                    let mut fahrenheit = String::new();

                    io::stdin()
                        .read_line(&mut fahrenheit)
                        .expect("Failed to read line");

                    let fahrenheit: f64 = match fahrenheit.trim().parse() {
                        Ok(num) => num,
                        Err(_) => continue,
                    };

                    let celsius: f64;

                    celsius = fahrenheit_to_celsius(fahrenheit);

                    println!("Temperature in celsius: {celsius}");
                    break;
                }
            }
            break;
        } else {
            println!("Invalid option!");
            continue;
        }
    }
}

fn temperature_converter(celsius: f64) -> f64 {
    (celsius * 9.0 / 5.0) + 32.0
}

fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}
