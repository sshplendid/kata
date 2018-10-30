// network::mod.rs
pub fn connect() {
    println!("connect to network!");
}

pub mod server;

pub mod client {
    pub fn connect() {
        println!("connect to network client!");
    }
}
