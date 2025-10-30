// Use an array of tuples (day, temp) and print the hottest day. 

fn main() {
    let weather_data = [
        ("Monday", 70),
        ("Tuesday", 75),
        ("Wednesday", 80),
        ("Thursday", 78),
        ("Friday", 85),
        ("Saturday", 90),
        ("Sunday", 88),
    ];

    let mut hottest_day = weather_data[0];

    for &(day, temp) in &weather_data {
        if temp > hottest_day.1 {
            hottest_day = (day, temp);
        }
    }

    println!("The hottest day is {} with a temperature of {}°F.", hottest_day.0, hottest_day.1);
}