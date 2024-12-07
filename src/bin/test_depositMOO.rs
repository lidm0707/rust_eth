use ethers::prelude::*;
use std::convert::TryFrom;
use std::sync::Arc;
// // use dotenv::dotenv;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    abigen!(
        MOOCOIN,
        "./src/build/src_contract_ERC20_sol_MooCoin.abi" // Path to the ABI file
    );
    // Set up the provider and wallet
    let provider = Provider::<Http>::try_from("http://127.0.0.1:8545")?;
    let wallet: LocalWallet =
        "0x370b655fbd7677d37190504eb3231a976adf2bc6e229b7b8fdf8d2339bf0bd63".parse()?;
    let chain_id = 1337u64; // Set to your local test network's chain ID (e.g., 1337 for Ganache)
    let wallet = wallet.with_chain_id(chain_id);
    let client = Arc::new(SignerMiddleware::new(provider, wallet));

    // Address of the deployed SwapPool contract
    let contract_address: Address = "0x2afd9dbb6363f0fdf44b43c28f38ef5e42a556eb".parse()?;
    let moocoin = MOOCOIN::new(contract_address, client.clone());

    // 1. Deposit ETH
    let mint_amount = U256::from(1_000_000_000_000_000_000u64); // 1 ETH in Wei
    let binding = moocoin.deposit().value(mint_amount);
    let tx = binding.send().await?;
    println!("Deposit transaction hash: {:?}", tx.tx_hash());

    Ok(())
}
