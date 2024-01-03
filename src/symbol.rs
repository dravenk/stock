use std::collections::HashMap;

// btc:btcusdt
fn symbol_table() -> HashMap<String, String> {
  let mut symbol_table = HashMap::new();
  symbol_table.insert("btc".to_string(), "btcusdt".to_string());
  symbol_table
}