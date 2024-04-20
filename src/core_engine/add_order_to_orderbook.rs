use super::order::BuyOrSell;
impl OrderBook {
    pub fn add_order_to_orderbook(&mut self, order: Order) {
        // Check the order tpye wheter it is a buy or sell order
        let order_price = order.price;

        match oder.order_tpye {
            BuyOrSell::Buy => {
                // Check it the Prices exists in the buy_orders HashMap
                match self.buy_orders.get_mut(&order_price) {
                    Some(orders) => {
                        // If it exists, add the order to the existing price point
                        order.push(order);
                    }
                    None => {
                        // If it does not exixt, create a new price point and add the order
                        self.buy_order.insert(order_price, vec![order]);
                    }
                }
            }
            BuyOrSell::Sell => {
                // Check If the price exists in the sell_orders HashMap
                match self.sell_orders.get_mut(&order_price) {
                    Some(orders) => {
                        // If it exists, add the order to the existing price point
                        orders.push(order);
                    }
                    None => {
                        // If it doesn't exist, create a new price point and add the order
                        self.sell_order.insert(order_price, vec![order]);
                    }
                }
            }
        }
    }
}
