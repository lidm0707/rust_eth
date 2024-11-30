
use ethers::providers::{Http, Middleware, Provider};
use eyre::Result;

// RPC และ API URL
const OPTIMISM_RPC: &str = "https://mainnet.optimism.io";

#[tokio::main]
async fn main() -> Result<()> {
    // สร้างตัวเชื่อมต่อกับ Optimism RPC
    let provider = Provider::<Http>::try_from(OPTIMISM_RPC)?;

    // ดึงราคาก๊าซปัจจุบันจาก Optimism
    let gas_price = provider.get_gas_price().await?;
    println!("Gas Price (Wei): {}", gas_price);

    // แปลงราคาก๊าซจาก Wei -> GWei (1 GWei = 10^9 Wei)
    // แปลงราคาก๊าซจาก GWei -> eth (1 eth = 10^9 GWei)    
    let gas_price_gwei = gas_price.as_u64() as f64 / 1_000_000_000.0;
    let gas_price_eth = gas_price.as_u64() as f64 / 1_000_000_000_000_000_000.0;


    // แสดงผลลัพธ์
    println!("Optimism Network Gas Prices:");
    println!("Gas Price: {gas_price_gwei:.5} Gwei");
    println!("gas eth: {gas_price_eth:.12} Eth");



    Ok(())
}


// fn eth_to_wei(eth_amount: f64) -> U256 {
//     let wei_per_eth: u128 = 10u128.pow(18);
//     let wei_amount = (eth_amount * wei_per_eth as f64) as u128;
//     U256::from(wei_amount)
// }


// ฟังก์ชันสำหรับดึงราคาของ ETH จาก API
// async fn fetch_eth_price() -> Result<f64> {
//     // ส่งคำขอไปที่ CoinGecko API
//     let response = reqwest::get(COINGECKO_PRICE_API).await?;
//     let price_data: serde_json::Value = response.json().await?;

//     println!("check get {:?}", price_data);

//     // ดึงราคาของ ETH ใน USD จาก JSON
//     let eth_price = price_data["ethereum"]["usd"]
//         .as_f64()
//         .expect("Failed to fetch ETH price");

//     Ok(eth_price)
// }

// // ฟังก์ชันคำนวณต้นทุนก๊าซ (Gas Cost) เป็น USD
// fn calculate_gas_cost_usd(gas_price: U256, eth_price: f64) -> f64 {
//     // กำหนดค่า Gas Limit มาตรฐาน
//     let standard_gas_limit = 21_000;

//     // แปลง Gas Cost จาก Wei -> ETH:
//     // gas_cost_eth = (gas_price (Wei) * gas_limit) / 10^18
//     let gas_cost_eth =
//         gas_price.as_u128() as f64 * standard_gas_limit as f64 / 1_000_000_000_000_000_000.0;

//     // คำนวณต้นทุนก๊าซใน USD:
//     // gas_cost_usd = gas_cost_eth * eth_price
//     gas_cost_eth * eth_price
// }
