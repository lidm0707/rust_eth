
use rust_eth::service::deploy_contract::deploy;
use std::{ fs::File, io::Read };

const GANACHE_URL: &str = "http://127.0.0.1:8545"; // RPC URL (local Ganache)
const PRIVATE_KEY: &str = "0x370b655fbd7677d37190504eb3231a976adf2bc6e229b7b8fdf8d2339bf0bd63"; // Replace with your private key

fn load_file(path: &str) -> String {
    let mut file = File::open(path).expect("Unable to open file");
    let mut content = String::new();
    file.read_to_string(&mut content).expect("Unable to read file");
    content
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    println!("start ...");
    let abi = load_file("./src/contract_build/src_contract_ERC20_sol_MooCoin.abi");
    let bytecode = load_file("./src/contract_build/src_contract_ERC20_sol_MooCoin.bin");
    let _ = deploy(&GANACHE_URL, &PRIVATE_KEY,&abi,&bytecode).await?;
    Ok(())
}
