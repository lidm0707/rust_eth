use ethers::prelude::*;
use std::convert::TryFrom;
use std::sync::Arc;
// // use dotenv::dotenv;

abigen!(
    SwapPool,
    "./src/abi/SwapPool.json" // Path to the ABI file
);

#[tokio::main]
async fn main() -> eyre::Result<()> {

    // Set up the provider and wallet
    let provider = Provider::<Http>::try_from("http://127.0.0.1:8545")?;
    let wallet: LocalWallet = "0x370b655fbd7677d37190504eb3231a976adf2bc6e229b7b8fdf8d2339bf0bd63".parse()?;
    let chain_id = 1337u64; // Set to your local test network's chain ID (e.g., 1337 for Ganache)
    let wallet = wallet.with_chain_id(chain_id);
    let client = Arc::new(SignerMiddleware::new(provider, wallet));

    // Address of the deployed SwapPool contract
    let contract_address: Address = "0x2afd9dbb6363f0fdf44b43c28f38ef5e42a556eb".parse()?;
    let swap_pool = SwapPool::new(contract_address, client.clone());

    // 1. Deposit ETH
    let deposit_amount = U256::from(1_000_000_000_000_000_000u64); // 1 ETH in Wei
    let binding = swap_pool
        .deposit()
        .value(deposit_amount);
    let tx = binding
        .send()
        .await?;
    println!("Deposit transaction hash: {:?}", tx.tx_hash());

    // 2. Swap balances
    let recipient: Address = "0x83b0b2Db521f68217D3A4ee04e3492c8D9cA239A".parse()?;
    let swap_amount = U256::from(500_000_000_000_000_000u64); // 0.5 ETH in Wei
    let binding = swap_pool
        .swap(recipient, swap_amount);
    let tx = binding
        .send()
        .await?;
    println!("Swap transaction hash: {:?}", tx.tx_hash());

    // 3. Withdraw ETH
    let withdraw_amount = U256::from(500_000_000_000_000_000u64); // 0.5 ETH in Wei
    let binding = swap_pool
        .withdraw(withdraw_amount);
    let tx = binding
        .send()
        .await?;
    println!("Withdraw transaction hash: {:?}", tx.tx_hash());

    Ok(())
}
