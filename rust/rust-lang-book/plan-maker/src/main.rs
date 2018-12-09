use std::thread;
use std::time::Duration;
use std::env;

fn main() {
    let args:Vec<String> = env::args().collect();

    let simulated_user_specified_value: u32 = args[1].parse().unwrap();
    let simulated_random_number: u32 = args[2].parse().unwrap();

    println!("{} {}", simulated_user_specified_value, simulated_random_number);

    generate_workout(simulated_user_specified_value, simulated_random_number);
}

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    println!("finished... intensity: {}", intensity);
    intensity
}

fn generate_workout(intensity: u32, random_number: u32) {
    let expensive_result = simulated_expensive_calculation(intensity);
    
    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result);
        println!("Next, do {} situps!", expensive_result);
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", expensive_result);
        }
    }
}
