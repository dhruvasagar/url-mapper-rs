use crate::config::CONFIG;

mod config;

fn main() {
    println!(
        "host: {}, port: {}, database.url: {}",
        CONFIG.host, CONFIG.port, CONFIG.database.url
    );
}
