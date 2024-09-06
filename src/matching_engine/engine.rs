use super::orderbook::{self, Order, Orderbook};
use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
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

    pub fn to_string(self) -> String {
        format!("{}_{}", self.base, self.quote)
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

    pub fn add_new_market(&mut self, pair: TradingPair) {
        self.orderbooks.insert(pair.clone(), Orderbook::new());
        println!("opening new orderbook for market {:?}", pair.to_string());
    }

    pub fn place_limit_order(
        &mut self, 
        pair: TradingPair, 
        price: f64, 
        order: Order
    ) -> Result<(), String> {
        match self.orderbooks.get_mut(&pair) {
            // market exists, add order to the appropriate orderbook
            Some(orderbook) => {
                orderbook.add_order(price, order);
                
                println!("placed limit order at price level {}", price);

                Ok(())
            }
            // order book for the given market does not exist
            None => Err(format!(
                "the orderbook for the given trading pair ({}) does not exist",
                pair.to_string()
            )),
        }
    }
}





 

