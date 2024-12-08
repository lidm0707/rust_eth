use dotenv::dotenv;
use rust_eth::service;
use std::env;

fn main() {
    dotenv().ok();
    let mnemonic = env::var("mnemonic").unwrap_or("not found mnemonic".to_owned());
    println!("{mnemonic}");
    let _ = service::mock_chain::serve_block(&mnemonic);
}
