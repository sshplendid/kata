use std::env;
use std::thread;
use std::time::Duration;

fn main() {
    let args: Vec<String> = env::args().collect();

    let simulated_user_specified_value: u32 = args[1].parse().unwrap();
    let simulated_random_number: u32 = args[2].parse().unwrap();

    println!(
        "{} {}",
        simulated_user_specified_value, simulated_random_number
    );

    generate_workout(simulated_user_specified_value, simulated_random_number);
}

/*
fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    println!("finished... intensity: {}", intensity);
    intensity
}
*/

fn generate_workout(intensity: u32, random_number: u32) {
    //let expensive_result = simulated_expensive_calculation(intensity);

    let mut expensive_result = Cacher::new(
    |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", expensive_result.value(intensity));
        }
    }
}

struct Cacher<T>
    where T: Fn(u32) -> u32
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
    where T: Fn(u32) -> u32
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            },
        }
    }
}

#[test]
fn call_with_same_value() {
    let mut c = Cacher::new(|a| a);

    let v1 = c.value(1);

    assert_eq!(v1, 1);
}

#[test]
fn call_with_different_value() {
    let mut c = Cacher::new(|a| a);

    let v1 = c.value(1);
    let v1 = c.value(2);

    assert_ne!(v1, 2);
}

#[test]
fn lexical_scope_test() {
    let x = 4;

    let equal_to_x = |z| z == x;
    //fn equal_to_x(z: i32) -> bool { z == x }

    let y = 4;

    assert!(equal_to_x(y));
}

#[test]
fn move_test() {
    let x = vec![1, 2, 3];

    let equal_to_x = move |z| z == x;

    //println!("can't use x here: {:?}", x);

    let y = vec![1, 2, 3];

    assert!(equal_to_x(y));
}
