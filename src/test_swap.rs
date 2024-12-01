use ethers::{
    prelude::*,
    types::{U256, Address, H256},
};
use std::{convert::TryFrom, error::Error, sync::Arc};

const GANACHE_URL: &str = "http://127.0.0.1:8545";  // Ganache RPC URL
const PRIVATE_KEY: &str = "0x370b655fbd7677d37190504eb3231a976adf2bc6e229b7b8fdf8d2339bf0bd63"; // Account 0's private key
const CONTRACT_ADDRESS: &str = "0x2afd9dbb6363f0fdf44b43c28f38ef5e42a556eb";  // Deployed contract address

// Example token addresses (replace with actual deployed token addresses)
const FROM_TOKEN_ADDRESS: &str = "0x...";  // Replace with actual ERC20 token address
const TO_TOKEN_ADDRESS: &str = "0x...";    // Replace with the token you want to receive

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Connect to Ganache
    let provider = Provider::<Http>::try_from(GANACHE_URL)?;

    // Create wallet and client
    let wallet: LocalWallet = PRIVATE_KEY.parse()?;
    let client = Arc::new(SignerMiddleware::new(provider.clone(), wallet));

    // Instantiate the contract at the given address
    let contract = Contract::new(
        CONTRACT_ADDRESS.parse()?,  // Contract address
        serde_json::from_slice(include_bytes!("./abi/SwapPool.json"))?,  // ABI of your contract
        client.clone()
    );

    // The amount you want to swap (in wei, 1 token = 10^18 wei for ERC20)
    let amount: U256 = U256::from_dec_str("1000000000000000000")?;  // 1 token

    // Approve the contract to spend the `from_token`
    let token_contract = Contract::new(
        FROM_TOKEN_ADDRESS.parse()?,
        serde_json::from_slice(include_bytes!("./abi/ERC20.json"))?,  // ERC20 ABI
        client.clone()
    );

    let approve_tx = token_contract
        .method::<_, H256>("approve", (CONTRACT_ADDRESS, amount))?  // Approve the swap contract to spend the tokens
        .gas(U256::from(100000))
        .send()
        .await?;

    println!("Approval transaction sent: {:?}", approve_tx);

    // Now that approval is done, call the swap function
    let slippage: U256 = U256::from(1); // Example slippage value (1%)
    let gas_price = provider.get_gas_price().await?; // Get current gas price

    let swap_tx = contract
        .method::<_, H256>("swap", (FROM_TOKEN_ADDRESS, TO_TOKEN_ADDRESS, amount, slippage))?  // Adjust the swap method name and args based on your contract's ABI
        .gas(U256::from(300000))
        .gas_price(gas_price)
        .send()
        .await?;

    println!("Swap transaction sent: {:?}", swap_tx);

    // Wait for transaction receipt
    let receipt = swap_tx.await?.ok_or("Transaction failed")?;
    println!("Transaction receipt: {:?}", receipt);

    Ok(())
}