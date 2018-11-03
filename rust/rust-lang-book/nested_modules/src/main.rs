pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules() {
                println!("function called.");
            }
        }
    }
}

fn main() {
    case_use();
    case_use_enum();
}

use a::series::of;
use a::series::of::nested_modules;

fn case_use() {
    // 아래 구문들은 동일한 결과를 갖는다.
    a::series::of::nested_modules();
    of::nested_modules();
    nested_modules();
}

enum TrafficLight {
    Red,
    Yellow,
    Green,
}

use TrafficLight::{Red, Yellow};
use TrafficLight::*;

fn case_use_enum() {
    let red = Red;
    let yello = Yellow;
    let green = TrafficLight::Green;
}
