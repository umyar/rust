use std::io::stdin;

enum TemperatureDesignation {
    Farenheit,
    Celsius,
}

// TODO:
// 1) check possible values of temperature (add kinda error for temperatures otside of possible values)

// QUESTIONS
// 1) can I just write integer numbers in expressions with float numbers?

fn main() {
    fn temperature_converter(temperature: f64, to_designation: TemperatureDesignation) -> f64 {
        match to_designation {
            TemperatureDesignation::Celsius => (temperature - 32.0) * 5.0 / 9.0,
            TemperatureDesignation::Farenheit => (temperature * 9.0 / 5.0) + 32.0,
        }
    }

    fn temperature_converter_io() {
        let mut temperature = String::new();
        let mut designation = String::new();

        println!("Type in your temperature:");

        stdin()
            .read_line(&mut temperature)
            .expect("Failed to read temperature");

        let temperature: f64 = match temperature.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                todo!();
            }
        };

        println!("And it is in Celsius or Farenheit? Type in C or F (lowercase also acceptable):");

        stdin()
            .read_line(&mut designation)
            .expect("Failed to parse designation");

        match designation.trim().to_lowercase().as_str() {
            "f" => println!(
                "{temperature}°F = {:.2}°C",
                temperature_converter(temperature, TemperatureDesignation::Celsius)
            ),
            "c" => println!(
                "{temperature}°C = {:.2}°F",
                temperature_converter(temperature, TemperatureDesignation::Farenheit)
            ),
            &_ => todo!("Damn... I can only Celsius or Farenheit"),
        };
    }

    temperature_converter_io();
}
