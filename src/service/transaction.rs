use abi::Abi;
use ethers::prelude::*;
use std::sync::Arc;

pub enum MethodContract {
    Deposit,
    Withdraw,
    Transfer,
    Balance,
}

impl MethodContract {
    pub fn as_str(&self) -> &'static str {
        match self {
            MethodContract::Deposit => "deposit",
            MethodContract::Withdraw => "withdraw",
            MethodContract::Transfer => "transfer",
            MethodContract::Balance => "balanceOf",
        }
    }
}

pub async fn use_contract(
    provider: Arc<Provider<Http>>,
    chain_id: u64,
    from: &LocalWallet,
    to: Address,
    json: &str,
    method: &MethodContract,
    amount: U256
) -> Result<(), anyhow::Error> {
    println!("Start {}", method.as_str());

    // โหลด ABI
    let abi: Abi = serde_json::from_str(json).map_err(|e| {
        eprintln!("Error parsing ABI: {:?}", e);
        anyhow::anyhow!("Failed to parse ABI")
    })?;

    // ตั้งค่า Wallet
    let wallet = from.clone().with_chain_id(chain_id);
    let client = Arc::new(SignerMiddleware::new(Arc::clone(&provider), wallet));

    // สร้าง Contract Instance
    let contract = Contract::new(to, abi, Arc::clone(&client));

    // Match method เพื่อเรียกฟังก์ชันที่เหมาะสม
    match method {
        MethodContract::Deposit => {
            println!("Calling deposit with value: {:?}", amount);
            contract.method::<(), ()>(&method.as_str(), ())?.value(amount).send().await?;
            println!("Transaction sent successfully: {:?}", contract.address());
        }
        MethodContract::Withdraw => {
            println!("Calling withdraw with amount: {:?}", amount);
            contract.method::<(), ()>(&method.as_str(), ())?.value(amount).send().await?;
            println!("Transaction sent successfully: {:?}", contract.address());
        }
        MethodContract::Transfer => {
            // ตัวอย่างสำหรับ Transfer (ต้องเพิ่มพารามิเตอร์ใหม่)
            println!("Transfer not implemented yet");
        }
        MethodContract::Balance => {
            let balance: U256 = contract
                .method::<Address, U256>(&method.as_str(), from.clone().address())?
                .call().await?;
            println!("Balance retrieved successfully: {:?}", balance);
            println!("Balance Eth {:?}",Arc::clone(&provider).get_balance(from.address(), None).await?);
        }
    }
    Ok(())
}
