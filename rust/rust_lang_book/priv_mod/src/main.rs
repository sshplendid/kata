mod outermost {
    pub fn middle_function() {
        println!("outermost::middlefunction called.");
        inside::inner_function();
    }

    fn middle_secret_function() {
        println!("outermost::middle_secret_function called.");
        inside::inner_function();
    }

    mod inside {
        pub fn inner_function() {
            println!("outermost::inside::inner_function called.");
            ::outermost::middle_function();
            ::outermost::middle_secret_function();
        }

        fn secret_funcction() {
            println!("outermost::inside::secret_function called.");
        }
    }
}

fn try_me() {
    outermost::middle_function();
    /*
    outermost::middle_secret_function();
    outermost::inside::inner_function();
    outermost::inside::secret_function();
    */
}
fn main() {
    try_me();
}
