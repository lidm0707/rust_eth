use ethers::{
    abi::Address,
    providers::{Http, Middleware, Provider},
    types::{transaction::eip2718::TypedTransaction, Eip1559TransactionRequest, U256},
};
use eyre::Result;
use std::str::FromStr;

const OPTIMISM_RPC: &str = "https://mainnet.optimism.io";

#[tokio::main]
async fn main() -> Result<()> {
    // สร้างตัวเชื่อมต่อกับ Optimism RPC
    let provider = Provider::<Http>::try_from(OPTIMISM_RPC)?;

    // สร้าง Address จาก String
    let from = Address::from_str("0x8A1B3C4D5E6F70890123456789ABCDEF01234567")?;
    let to = Address::from_str("0x9B2C4D5E6F7A8090123456789ABCDEFF12345678")?;

    // ตั้งค่าการทำธุรกรรม EIP-1559
    let tx = Eip1559TransactionRequest::new()
        .from(from) // Address ที่ส่ง
        .to(to) // Address ที่รับ
        .value(eth_to_wei(1.00)) // ค่า ETH (1 ETH)
        .gas(U256::from(21_000)) // Gas limit complex bas is not wei and eth
        .max_priority_fee_per_gas(gwei_to_wei(2)) // Max priority fee (2 GWei)
        .max_fee_per_gas(gwei_to_wei(50)); // Max fee per gas (50 GWei)

    // แปลง EIP-1559 Transaction เป็น TypedTransaction
    let tx_typed: TypedTransaction = tx.into();

    // คำนวณ Gas ที่ต้องใช้
    match provider.estimate_gas(&tx_typed, None).await {
        Ok(gas_estimate) => {
            // การประเมิน gas สำเร็จ
            println!("Estimated Gas: {}", gas_estimate);
        }
        Err(error) => {
            // การประเมิน gas ล้มเหลว
            eprintln!("Failed to estimate gas: {}", error);
        }
    };
    

    Ok(())
}

fn eth_to_wei(eth: f64) -> U256 {
    // 1 ETH = 10^18 Wei
    let wei_per_eth = 1_000_000_000_000_000_000u128;
    let wei_amount = (eth * wei_per_eth as f64) as u128;
    U256::from(wei_amount)
}

fn gwei_to_wei(gwei: u64) -> U256 {
    let wei_per_gwei = 1_000_000_000u64; // 1 GWei = 10^9 Wei
    U256::from(gwei) * U256::from(wei_per_gwei)
}
