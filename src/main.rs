use std::collections::HashMap;

#[derive(Debug)]
enum BidOrAsk {
    Bid,
    Ask,
}

#[derive(Debug)]
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

                // Here, we are matching bigs against the limit and if a match
                // is not found, then a new limit is created and the orders are
                // added under that price limit
                match self.bids.get_mut(&price) {
                    // If limit already exists then proceed to add the order
                    // without creating a new limit
                    Some(limit) => limit.add_order(order),
                    // println!("Already got a limit!"),
                    None => {
                        let mut limit = Limit::new(price);
                        limit.add_order(order);
                        self.bids.insert(price, limit);
                    }
                }
            }
            BidOrAsk::Ask => {}
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
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
    fn new(price: Price) -> Limit {
        Limit {
            price,
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
    // let mut limit = Limit::new(65.3);

    let buy_order_from_alice = Order::new(BidOrAsk::Bid, 5.5);
    let buy_order_from_bob = Order::new(BidOrAsk::Bid, 2.45);

    // let sell_order = Order::new(BidOrAsk::Ask, 2.45);

    let mut orderbook = Orderbook::new();

    // Adds it as limit does not exist
    orderbook.add_order(4.4, buy_order_from_alice);
    // Throws that already got a limit nad does not add it
    orderbook.add_order(4.4, buy_order_from_bob);

    // limit.add_order(buy_order);
    // limit.add_order(sell_order);

    println!("{:?}", orderbook);
}
