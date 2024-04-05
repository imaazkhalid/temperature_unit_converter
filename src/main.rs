use std::io;

fn main() {
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

        println!("{fahrenheit}");
        break;
    }
}

fn temperature_converter(celsius: f64) -> f64 {
    (celsius * 9.0 / 5.0) + 32.0
}
