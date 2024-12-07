use abi::Abi;
use ethers::prelude::*;
use ethers::providers::{ Provider, Http };
use std::convert::TryFrom;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Set up the provider (replace with your Ethereum node URL)
    let provider: Arc<Provider<Http>> = Arc::new(
        Provider::<Http>::try_from("http://127.0.0.1:8545")?
    );
    // let chain_id = provider.get_chainid().await?;

    // Specify the wallet address (replace with the address you want to check)
    let token_contract_address: Address = "0x2afd9dbb6363f0fdf44b43c28f38ef5e42a556eb".parse()?;

    // Wallet address whose token balance you want to check
    let wallet_address: Address = "0x0a69428ed9206a87C09fF3b0B8FBaa464AAf4794".parse()?;

    // Load the ERC-20 ABI (Minimal ABI for balanceOf)
    let mut file = File::open("./src/contract_build/src_contract_ERC20_sol_MooCoin.abi").expect(
        "Unable to open file"
    );
    let mut content = String::new();
    file.read_to_string(&mut content).expect("Unable to read file");

    let erc20_abi: Abi = serde_json::from_str(&content)?;
    // Initialize the contract
    let contract: ContractInstance<Arc<Provider<Http>>, _> = Contract::new(
        token_contract_address,
        erc20_abi,
        Arc::clone(&provider)
    );

    // Call the `balanceOf` function
    let balance: U256 = contract
        .method::<Address, U256>("balanceOf", wallet_address)?
        .call().await?;

    // USDC uses 6 decimals; adjust accordingly
    let balance_in_tokens = ethers::utils::format_units(balance, 6)?;
    println!("Balance: {} moocoin", balance_in_tokens);

    let eth_balance: U256 = Arc::clone(&provider).get_balance(wallet_address, None).await?;
    let formatted_eth_balance = ethers::utils::format_units(eth_balance, 18)?;
    println!("ETH Balance: {} ETH", formatted_eth_balance);

    Ok(())
}
