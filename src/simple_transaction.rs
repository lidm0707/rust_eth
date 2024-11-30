#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
use ethers::{
    middleware::gas_oracle::GasCategory,
    prelude::{Address, LocalWallet, Middleware, Provider, Signer, TransactionRequest, U256},
    signers::Wallet,
    types::transaction::eip2718::TypedTransaction,
    utils::Ganache,
};
use eyre::{ContextCompat, Result};
use hex::ToHex;
use std::{convert::TryFrom, time::Duration};
#[allow(dead_code)]
#[tokio::main]
async fn main() -> Result<()> {
    let provider =
        Provider::try_from("http://localhost:8545/")?.interval(Duration::from_millis(10));

    // Check balance of another address
    let first_address = "0xd03b147131d42651AEdD21687B82B231349013d3".parse::<Address>()?;
    let other_address = "0x301cb85D746af369b496f678C0ed36919F028CB6".parse::<Address>()?;
    let other_balance = provider.get_balance(other_address, None).await?;
    println!(
        "Balance for address {}: {} ETH",
        other_address,
        ethers::utils::format_ether(other_balance)
    );

    // Create a transaction to transfer 1000 wei to `other_address`
    let tx: TransactionRequest =
        TransactionRequest::pay(other_address, U256::from(1000u64)).from(first_address);
    let gas_price = provider.get_gas_price().await?;
    println!("Estimated gas price: {:?}", gas_price);
    let type_tran = TypedTransaction::Legacy(tx.clone());
    let gas_estimate = provider.estimate_gas(&type_tran, None).await?;
    println!("Estimated gas: {:?}", gas_estimate);

    // Send the transaction and wait for receipt
    let receipt = provider
        .send_transaction(tx.clone(), None)
        .await?
        .log_msg("Pending transfer")
        .await?
        .context("Missing receipt")?;
    let gas = &tx.gas_price;

    println!("TX Gas {:?}", gas);

    println!(
        "TX mined in block {}",
        receipt.block_number.context("Can not get block number")?
    );
    println!(
        "Balance of {} {}",
        other_address,
        convert_balance_to_ether(provider.get_balance(other_address, None).await?)
    );

    println!(
        "Balance of {} {}",
        first_address,
        convert_balance_to_ether(provider.get_balance(first_address, None).await?)
    );

    Ok(())
}

fn convert_balance_to_ether(balance: U256) -> f64 {
    // Convert U256 to a u128 (only safe for values that fit in 128 bits)
    let balance_as_f64 = balance.as_u128() as f64;
    // Scale down to ether (divide by 10^18 to convert from wei to ether)
    balance_as_f64 / 1e18
}
