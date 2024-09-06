use super::orderbook::Orderbook;
use std::collections::HashMap;

// e.g. BTC is the base, USD is the quote
pub struct TradingPair {
    base: String,
    quote: String,
}

impl TradingPair {
    pub fn new(base: String, quote: String) -> TradingPair {
        TradingPair {
            base,
            quote
        }
    }
}

pub struct MatchingEngine {
    orderbooks: HashMap<TradingPair, Orderbook>,
}

impl MatchingEngine {
    pub fn new() -> MatchingEngine {
        MatchingEngine {
            orderbooks:HashMap::new(),
        }
    }

   // pub fn add_new_market()
}
 

