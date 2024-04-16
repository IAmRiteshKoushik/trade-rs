enum BidOrAsk {
    Bid,
    Ask,
}

#[derive(Debug)]
struct Price {
    integral: u64,
    fractional: u64,
    scalar: u64,
}

impl Price {
    fn new(price: f64) -> Price {
        let scalar = 100_000;
        let integral = price as u64; // Casting a float -> int (only integral)
        let fractional = ((price % 1.0) * scalar as f64) as u64;
        // Taking module by 1.0 to find the fractional part
        // Then scaling it up using the scalar variable for a precision
        // Finally casting it as an unsigned integer
        Price {
            scalar,
            integral,
            fractional,
        }
    }
}

struct Limit {
    // f64 has got problems with hashing functions hence a custom implementation
    // of the price structure is more suitable
    // price: f64,
    price: Price,
}

struct Order {
    size: f64,
    bid_or_ask: BidOrAsk,
}

impl Order {
    fn new(bid_or_ask: BidOrAsk, size: f64) -> Order {
        Order { bid_or_ask, size }
    }
}

fn main() {
    let price = Price::new(50.5);
    println!("{:?}", price);
}
