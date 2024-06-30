use super::orderbook::{Order, Orderbook};
use std::collections::HashMap;

// BTC / USD = Base : Quote
#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct TradingPair {
    base: String,
    quote: String,
}

impl TradingPair {
    pub fn new(base: String, quote: String) -> TradingPair {
        TradingPair { base, quote }
    }

    // Improving readability by adding a method which can return a string
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
            orderbooks: HashMap::new(),
        }
    }

    // You would like to have separate order-book for BTC/USD and ETH/USD.
    // So there are multiple markets to create
    pub fn add_new_market(&mut self, pair: TradingPair) {
        self.orderbooks.insert(pair.clone(), Orderbook::new());
        println!("Opening new orderbook for market {:?}", pair.to_string());
    }

    pub fn place_limit_order(
        &mut self,
        pair: TradingPair,
        price: f64,
        order: Order,
    ) -> Result<(), String> {
        match self.orderbooks.get_mut(&pair) {
            // If the market exists, then we add the order to the required
            // order book and if it doesnt exist then we cancel it
            Some(orderbook) => {
                orderbook.add_order(price, order);
                println!("placed limit order price level {:?}", price);
                Ok(())
            }
            None => Err(format!(
                "the order book for the given trading pair ({}) does not exist",
                pair.to_string()
            )),
        }
    }
}
