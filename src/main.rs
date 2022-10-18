use std::io;

fn main() {
    println!("This tool converts temperatures between Fahrenheit and Celsius.\nPlease enter your temperature value, ending in F or C respectively (e.g., 32C, 86.6F).");

    const FAHRENHEIT_TEMPERATURE_LABEL: char = 'F';
    const CELSIUS_TEMPERATURE_LABEL: char = 'C';

    let allowed_temperatures: [char; 2] = [FAHRENHEIT_TEMPERATURE_LABEL, CELSIUS_TEMPERATURE_LABEL];

    let mut temperature: String = String::new();

    let temperature_type: char = loop {
        io::stdin()
        .read_line(&mut temperature)
        .expect("Failed to read line");

        let temperature_type: char = extract_temperature_label(&temperature);

        if !allowed_temperatures.contains(&temperature_type) {
            println!("The format you have written is incorrect. You should write the temperature and the unit concatenated. Example: 32F, 10C, etc.");

            temperature.clear();
            continue;
        }

        break temperature_type
    };

    let temperature: f32 = extract_temperature_without_label(temperature, temperature_type);
    let destionation_temperature_type: char = extract_destionation_temperature_label_from_source(allowed_temperatures, temperature_type);

    println!("You have entered: {temperature}{temperature_type}. It will now be converted to {destionation_temperature_type}.");

    let temperature: f32 = match temperature_type {
        FAHRENHEIT_TEMPERATURE_LABEL => {
            convert_fahrenheit_to_celsius(temperature)
        },
        CELSIUS_TEMPERATURE_LABEL => {
            convert_celsius_to_fahrenheit(temperature)
        },
        _ => 0.0
    };

    println!("The temperature has been converted to: {:.1}{destionation_temperature_type}", temperature);
}

fn extract_destionation_temperature_label_from_source(allowed_temperatures: [char; 2], source_temperature_label: char) -> char {
    allowed_temperatures[allowed_temperatures.iter().position(|&x| x != source_temperature_label).unwrap()]
}

fn extract_temperature_label(temperature: &String) -> char {
    temperature
        .trim()
        .chars()
        .last()
        .unwrap()
}

fn extract_temperature_without_label(temperature: String, temperature_type: char) -> f32 {
    temperature
        .trim()
        .chars()
        .take_while(|x| *x != temperature_type)
        .collect::<String>()
        .parse()
        .unwrap()
}

fn convert_fahrenheit_to_celsius(fahrenheit_temperature: f32) -> f32 {
    (fahrenheit_temperature - 32.0) * 5.0/9.0
}

fn convert_celsius_to_fahrenheit(celsius_temperature: f32) -> f32 {
    (celsius_temperature * 9.0/5.0) + 32.0
}