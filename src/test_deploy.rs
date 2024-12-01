use abi::Abi;
use ethers::{
    prelude::*,
    types::{Bytes},
};
use std::{convert::TryFrom, sync::Arc, fs::File, io::Read};

const GANACHE_URL: &str = "http://127.0.0.1:8545"; // RPC URL (local Ganache)
const PRIVATE_KEY: &str = "0x370b655fbd7677d37190504eb3231a976adf2bc6e229b7b8fdf8d2339bf0bd63";       // Replace with your private key

fn load_file(path: &str) -> String {
    let mut file = File::open(path).expect("Unable to open file");
    let mut content = String::new();
    file.read_to_string(&mut content).expect("Unable to read file");
    content
}

#[tokio::main]
async fn main() -> eyre::Result<()> {
    // Load ABI and Bytecode
    // focus on taget floder
    let abi = load_file("./src/build/SwapPool_sol_SwapPool.abi");
    let bytecode = load_file("./src/build/SwapPool_sol_SwapPool.bin");
    

    // Connect to Ethereum node
    let provider = Provider::<Http>::try_from(GANACHE_URL)?;
    let chain_id = provider.get_chainid().await?;
    println!("Connected to chain ID: {}", chain_id);

    // Create a wallet
    let wallet: LocalWallet = PRIVATE_KEY.parse()?;
    let client = Arc::new(SignerMiddleware::new(provider, wallet.with_chain_id(chain_id.as_u64())));

    // Parse ABI and Bytecode
    let abi: Abi = serde_json::from_str(&abi)?;
    let bytecode = Bytes::from(hex::decode(bytecode.trim())?);

    // Deploy the contract
    println!("Deploying contract...");
    let factory = ContractFactory::new(abi, bytecode, client.clone());
    let deploy_tx = factory.deploy(())?; // Pass constructor args if needed
    let contract = deploy_tx.send().await?;

    println!("Contract deployed at: {:?}", contract.address());

    Ok(())
}
