use std::io::stdin;

enum TemperatureDesignation {
    Farenheit,
    Celsius,
}

fn main() {
    // Simple function with number and enum arguments
    // check possible values of temperature (add kinda error for temperatures otside of possible values)
    fn temperature_converter(temperature: i32, to_designation: TemperatureDesignation) -> i32 {
        match to_designation {
            TemperatureDesignation::Celsius => (temperature - 32) * 5 / 9,
            TemperatureDesignation::Farenheit => (temperature * 9 / 5) + 32,
        }
    }

    // Function through std::io

    fn temperature_converter_io() {}
}
