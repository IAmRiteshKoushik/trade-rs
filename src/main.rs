mod matching_engine;
use matching_engine::engine::{MatchingEngine, TradingPair};
use matching_engine::orderbook::{BidOrAsk, Order, Orderbook};

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

    let sell_order = Order::new(BidOrAsk::Ask, 6.5);
    orderbook.add_order(20.0, sell_order);

    // You can pretty-print using "#" instead of printing everything in a single
    // line. Usually, Debug statements need to be printed to a log file hence
    // a single line is required, but for printing to stdout, we can afford
    // to see a preetier format.
    // -----------------------------
    // println!("{:#?}", orderbook);
    // println!("{:?}", orderbook);

    // Creating a new market and adding a trading pair
    let mut engine = MatchingEngine::new();
    let pair = TradingPair::new("BTC".to_string(), "USDC".to_string());
    engine.add_new_market(pair.clone());

    let buy_order = Order::new(BidOrAsk::Bid, 6.5);
    // ------ PANIC ----------
    // let eth_pair = TradingPair::new("ETH".to_string(), "USD".to_string());
    // Because place_limit_order has a return type that we do not care about
    // we are advised to either declare an escape variable like _ or panic
    // ------ PANIC ----------
    // engine
    //     .place_limit_order(eth_pair, 10.000, buy_order)
    //     .unwrap();

    engine.place_limit_order(pair, 10.000, buy_order).unwrap();
}
