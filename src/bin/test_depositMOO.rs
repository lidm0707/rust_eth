use ethers::prelude::*;
use rust_eth::service::transaction::{ use_contract, MethodContract::{Deposit,Balance} };
use std::convert::TryFrom;
use std::fs::File;
use std::io::Read;
use std::sync::Arc;
// // use dotenv::dotenv;

fn load_file(path: &str) -> String {
    let mut file = File::open(path).expect("Unable to open file");
    let mut content = String::new();
    file.read_to_string(&mut content).expect("Unable to read file");
    content
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Set up the provider and wallet
    let provider = Arc::new(Provider::<Http>::try_from("http://127.0.0.1:8545")?);
    let from: LocalWallet =
        "0x370b655fbd7677d37190504eb3231a976adf2bc6e229b7b8fdf8d2339bf0bd63".parse()?;
    let chain_id = 1337u64; // Set to your local test network's chain ID (e.g., 1337 for Ganache)
    let to = "0x2afd9dbb6363f0fdf44b43c28f38ef5e42a556eb".parse()?;
    let json = load_file("./src/contract_build/src_contract_ERC20_sol_MooCoin.abi");
    let mint_amount = U256::from(1_000_000_000_000_000_000u64); // 1 ETH in Wei

    let _ = use_contract(Arc::clone(&provider), chain_id, from.clone(), to, &json, Deposit, mint_amount).await?;
    let _ = use_contract(provider, chain_id, from, to, &json, Balance, U256::from(0u64)).await?;
    Ok(())
}
