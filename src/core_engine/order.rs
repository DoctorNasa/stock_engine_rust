use rust_decimal::Decimal;
pub enum BuyOrSell {
    Buy,
    Sell,
}

pub struct Order {
    pub quantitiy: Decimal,
    pub price: Decimal,
    pub order_type: BuyOrSell,
}

impl Order {
    pub fn new(quantity: Decimal, price: Decimal, order_type: BuyOrSell) -> Order {
        Order {
            quantity,
            price,
            order_tpye,
        }
    }
}
