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
    // this function adds an order
    fn add_order(&mut self, price: f64, order: Order) {
        
        let price = Price::new(price);
       
        match order.bid_or_ask {
            BidOrAsk::Bid => match self.bids.get_mut(&price) {
                Some(limit) => limit.add_order(order),
                None => {
                    let mut limit = Limit::new(price);
                    limit.add_order(order);
                    self.bids.insert(price, limit);
                }
            },
            
            BidOrAsk::Ask => match self.bids.get_mut(&price) { 
                Some(limit) => limit.add_order(order),
                None => {
                    let mut limit = Limit::new(price);
                    limit.add_order(order);
                    self.asks.insert(price, limit);
                }
            },
        }
    }
}    


#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
struct Price {
    intergral: u64,
    fractional: u64,
    scalar: u64,
}

impl Price {
    fn new(price: f64) -> Price {
        let intergral = price as u64;
        let scalar = 100000;
        let fractional = ((price % 1.0) * scalar as f64) as u64;

        Price {
            scalar,
            intergral,
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
        Order { bid_or_ask, size }
    }
}

fn main() {

    let buy_order_from_alice = Order::new(BidOrAsk::Bid, 5.5);
    let buy_order_from_bob = Order::new(BidOrAsk::Bid, 2.45);

    let mut orderbook = Orderbook::new();
    orderbook.add_order(4.4, buy_order_from_alice);
    orderbook.add_order(4.4, buy_order_from_bob);

    let sell_order = Order::new(BidOrAsk::Ask, 6.5);
    orderbook.add_order(20.2, sell_order);

    println!("{:?}", orderbook);
}
