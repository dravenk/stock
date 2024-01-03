use crate::api;
use reqwest;


pub struct Price {
  pub symbol: String,
  pub price: String
}

impl Price {
  pub async fn get(symbol: &str) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::builder()
        .build()?;
    let request = client.request(reqwest::Method::GET, api::binance_api() + "ticker/price?symbol=" + symbol);
    let response = request.send().await?;
    let body = response.text().await?;
    Ok(body)
  }

  pub async fn get_btc_price() -> Result<String, Box<dyn std::error::Error>> {
    let body = Self::get("BTCUSDT").await?;
    Ok(body)
  }
}
