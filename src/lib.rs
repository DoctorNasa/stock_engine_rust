use std::{cmp::Ordering, io::BufRead};

use rust_decimal::Decimal;
use rust_decimal_macros::dec;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}
pub mod core_engine;
// write a test case
#[cfg(test)]
mod tests {
    use super::*;
    use core_engine::{
        order::{BuyOrSell, Order},
        orderbook::OrderBook,
    };
    use rust_decimal_macros::dec;
    #[test]
    fn text_add_order_to_orderbook() {
        // Initialze the new order_book
        let mut order_book = OrderBook::new();

        // Create some buy orders.
        let buy_order_1 = Order::new(dec!(35), dec!(690), BuyOrSell::Buy);
        let buy_order_2 = Order::new(dec!(20), dec!(685), BuyOrSell::Buy);
        let buy_order_3 = Order::new(dec!(15), dec!(690), BuyOrSell::Buy);

        // Create a new sell order.
        let sell_order_1 = Order::new(dec(10), dec!(700), BuyOrSell::Sell);
        let sell_order_2 = Oreder::new(dec!(25), dec!(705), BuyOrSell::Sell);
        let sell_order_3 = Order::new(dec!(30), dec!(700), BuyOrSell::Sell);

        // Add the order to the order_book
        order_book.add_order_to_orderbook(buy_order_2);
        order_book.add_order_to_orderbook(buy_order_3);
        order_book.add_order_to_orderbook(sell_order_1);
        order_book.add_order_to_orderbook(sell_order_2);
        order_book.add_order_to_orderbook(sell_order_3);
        assert_eq!(order_book.buy_orders.len(), 2);
        assert_eq!(order_book.sell_orders.len(), 2);
        assert_eq!(order_book.buy_orders.get(&dec!(690)).unwrap().len(), 2);
        assert_eq!(order_book.buy_orders.get(&dec!(685)).unwrap().len(), 1);
        assert_eq!(order_book.sell_orders.get(&dec!(700)).unwrap().len(), 2);
        assert_eq!(order_book.sell_orders.get(&dec!(705)).unwrap().len(), 1);
    }
    // fn it_works() {
    //     let result = add(2, 2);
    //     assert_eq!(result, 4);
    // }
}

impl OrderBook {
    pub fn best_buy_price(&self) -> Option<Decimal> {
        // Get the maximum price from the buy_orders HashMap
        self.buy_orders.keys().max().cloned()
    }

    pub fn best_sell_price(&self) -> Option<Decimal> {
        // Get the minimum price from the sell_orders HashMap
        self.sell_orders.keys().min().cloned()
    }

    pub fn buy_volume(&self) -> Option<Decimal> {
        // Calculate the total volume of the buy orders
        let buy_volome: Decimal = self
            .buy_orders
            .values()
            .flatten()
            .map(|order| order.quantity)
            .sum();
        Some(buy_volome)
    }

    pub fn sell_volome(&self) -> Option<Decimal> {
        // Calculate the total volume of the
        let sell_volume: Decimal = self
            .sell_orders
            .values()
            .flatten()
            .map(|order| order.quantity)
            .sum();
        Some(sell_volume)
    }
}

// Create a tracking buy volume ans sell volume of the stock.
#[Test]
fn test_prices_and_volume() {
    // Initialz the new order_book
    let mut order_book = OrderBook::new();

    // Create some buy orders.
    let buy_order_1 = Order::new(dec!(35), dec!(690), BuyOrSell::Buy);
    let buy_order_2 = Order::new(dec!(20), dec!(685), BuyOrSell::Buy);
    let buy_order_3 = Order::new(dec!(15), dec!(690), BuyOrSell::Buy);

    // Create some sell orders.
    let sell_order_1 = Order::new(dec!(10), dec!(700), BuyOrSell::Sell);
    let sell_order_2 = Order::new(dec!(25), dec!(705), BuyOrSell::Sell);
    let sell_order_3 = Order::new(dec!(30), dec!(700), BuyOrSell::Sell);

    // Add the orders to the order_book
    order_book.add_order_to_orderbook(buy_order_1);
    order_book.add_order_to_orderbook(buy_order_2);
    order_book.add_order_to_orderbook(buy_order_3);
    order_book.add_order_to_orderbook(sell_order_1);
    order_book.add_order_to_orderbook(sell_order_2);
    order_book.add_order_to_orderbook(sell_order_3);

    assert_eq!(order_book.best_buy_price().unwrap(), dec!(690));
    assert_eq!(order_book.best_sell_price().unwrap(), dec!(700));
    // Total Buying Order Quantity = 35+20+15
    assert_eq!(order_book.buy_volume().unwrap(), dec!(70));
    // Total Selling Order Quantity = 10+25+30
    assert_eq!(order_book.sell_volume().unwrap(), dec!(65));
}
