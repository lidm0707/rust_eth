use ethers::providers::{ Http, Middleware, Provider };
use eyre::Result;

// RPC และ API URL
const OPTIMISM_RPC: &str = "https://eth.llamarpc.com";

#[tokio::main]
async fn main() -> Result<()> {
    // สร้างตัวเชื่อมต่อกับ Optimism RPC
    let provider = Provider::<Http>::try_from(OPTIMISM_RPC)?;

    // ดึงราคาก๊าซปัจจุบันจาก Optimism
    let gas_price = provider.get_gas_price().await?;
    println!("Gas Price (Wei): {}", gas_price);

    // แปลงราคาก๊าซจาก Wei -> GWei (1 GWei = 10^9 Wei)
    // แปลงราคาก๊าซจาก GWei -> eth (1 eth = 10^9 GWei)
    let gas_price_gwei = (gas_price.as_u64() as f64) / 1_000_000_000.0;
    let gas_price_eth = (gas_price.as_u64() as f64) / 1_000_000_000_000_000_000.0;

    // แสดงผลลัพธ์
    println!("Optimism Network Gas Prices:");
    println!("Gas Price: {gas_price_gwei:.5} Gwei");
    println!("gas eth: {gas_price_eth:.12} Eth");

    Ok(())
}


