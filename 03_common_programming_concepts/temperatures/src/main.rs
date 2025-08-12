fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}

fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    celsius * 9.0 / 5.0 + 32.0
}

fn main() {
    let fahrenheit = 68.0;
    let celsius = fahrenheit_to_celsius(fahrenheit);
    println!("{fahrenheit}°F is {celsius}°C");

    let celsius = 20.0;
    let fahrenheit = celsius_to_fahrenheit(celsius);
    println!("{celsius}°C is {fahrenheit}°F");
}