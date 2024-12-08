use abi::Abi;
use ethers::{ prelude::*, types::Bytes };
use std::sync::Arc;

pub async fn deploy(
    url: &str,
    private_key: &str,
    abi: &str,
    bytecode: &str
) -> Result<Address, anyhow::Error> {
    let provider = Provider::<Http>::try_from(url).unwrap();
    let chain_id = &provider.get_chainid().await.unwrap();
    println!("Connected to chain ID: {}", chain_id);
    // Create a wallet
    let wallet: LocalWallet = private_key.parse().unwrap();
    let client = SignerMiddleware::new(provider, wallet.with_chain_id(chain_id.as_u64()));

    // Parse ABI and Bytecode
    let abi: Abi = serde_json::from_str(&abi).unwrap();
    let bytecode = Bytes::from(hex::decode(&bytecode.trim()).unwrap());

    // Deploy the contract
    println!("Deploying contract...");
    let factory = ContractFactory::new(abi, bytecode, Arc::new(client));
    let deploy_tx = factory.deploy(()).unwrap(); // Pass constructor args if needed
    let contract = deploy_tx.send().await.unwrap();

    println!("Contract deployed at: {:?}", contract.address());

    Ok(contract.address())
}
