use super::order::Order;
use rust_decimal::Decimal;
use std::collection::HashMMap;

pub struct OrderBook {
    // HashMap : [Key : Price, Value : All the orders at the price]
    pub buy_orders: HashMap<Decimal, Vec<Order>>,
    pub sell_orders: HashMap<Decimal, Vec<Order>>,
}

impl OrderBook {
    pub fn new() -> OrderBook {
        OrderBook {
            buy_orders: HashMap::new(),
            sell_orders: HashMap::new(),
        }
    }
}
