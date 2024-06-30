use std::collections::HashMap;

#[derive(Debug)]
enum BidOrAsk {
    Bid,
    Ask,
}

struct Orderbook {
    asks: HashMap<Price, Limit>,
    bids: HashMap<Price, Limit>,
}

impl Orderbook {
    fn new() -> Orderbook {
        Orderbook {
            asks: HashMap::new(),
            bids: HashMap::new(),
        }
    }

    // A limit order will sit in the order book but a market order would not
    fn add_order(&mut self, price: f64, order: Order) {
        match order.bid_or_ask {
            BidOrAsk::Bid => {
                let price = Price::new(price);
                let limit = self.bids.get_mut(&price);

                match limit {
                    Some(limit) => println!("Already got a limit!"),
                    None => println!("need to create a new limit!"),
                }
            }
            BidOrAsk::Ask => {}
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash)]
struct Price {
    integral: u64,
    fractional: u64,
    scalar: u64,
}

impl Price {
    fn new(price: f64) -> Price {
        let scalar = 100000;
        // Type-casting works with the "as" keyword. Here, we are only
        // interested in the integral part of the float
        let integral = price as u64;
        let fractional = ((price % 1.0) * scalar as f64) as u64;
        Price {
            scalar,
            integral,
            fractional,
        }
    }
}

#[derive(Debug)]
struct Limit {
    price: Price,
    orders: Vec<Order>,
}

impl Limit {
    fn new(price: f64) -> Limit {
        Limit {
            price: Price::new(price),
            orders: Vec::new(),
        }
    }

    fn add_order(&mut self, order: Order) {
        self.orders.push(order);
    }
}

#[derive(Debug)]
struct Order {
    size: f64,
    bid_or_ask: BidOrAsk,
}

impl Order {
    fn new(bid_or_ask: BidOrAsk, size: f64) -> Order {
        // If naming and arguments are same then you need not
        // explicitly declare them inside the struct and just get away
        // with doing it as follows
        Order { bid_or_ask, size }
    }
}

fn main() {
    // Testing it out
    let mut limit = Limit::new(65.3);
    let buy_order = Order::new(BidOrAsk::Bid, 5.5);
    let sell_order = Order::new(BidOrAsk::Ask, 2.45);
    limit.add_order(buy_order);
    limit.add_order(sell_order);
    println!("{:?}", limit);
}
