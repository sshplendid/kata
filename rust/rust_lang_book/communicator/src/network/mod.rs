// network::mod.rs
fn connect() {
    println!("connect to network!");
}

mod server;

mod client {
    fn connect() {
        println!("connect to network client!");
    }
}
