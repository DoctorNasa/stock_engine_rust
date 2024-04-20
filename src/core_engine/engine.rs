// create parameters as name,symbol, sector,exchainge
pub enum Market {
    ThaiMarket(ThaiExchange),
    HkMarket(HongkonExchange),
    UsMarket(USExchange),
    CryptoMarket(CryptoExchange),
}

pub enum ThaiExchange {
    SET,
}

pub enum HongkonExchange {
    HKEX,
}
pub enum USExchange {
    NASDAQ,
    NYSE,
}

pub enum CryptoExchange {
    WazirX,
    CoinDCX,
    Binance,
    Coinbase,
}

pub enum Sector {
    Technology,
    Finance,
    Banking,
    ArtToy,
    Healthcare,
    Energy,
    ConsumerDiscretionary,
    ConsumerStaples,
    Industrials,
    Materials,
    RealEstate,
    CommunicationServices,
    Utilities,
}

use super::orderbook::OrderBook;
use std::collections::HashMap;

pub struct Company {
    name: String,
    symbol: String,
    sector: Sector,
    market: Market,
}

impl Company {
    pub fn new(name: String, symbol: String, sector: Sector, market: Market) -> Company {
        Company {
            name,
            symbol,
            sector,
            market,
        }
    }
}

pub struct MatchingEngine {
    pub orderbooks: HashMap<Company, OrderBook>,
}

impl MachingEngine {
    pub fn new() -> MatchingEngine {
        MatchingEngine {
            orderbooks: HashMap::new(),
        }
    }
}

// create a methods list any new company or ght the order book of existing company.
#[derive(Hash, PartialEq, Eq, Clone)]
impl MatchingEngine {
    pub fn list_new_company(&mut self, company: Company) {
        let orderbook = OrderBook::new();
        self.orderbooks.insert(company, orderbook);
    }
    pub fn get_company_orderbook(&mut self, company: &Company) -> Option<&mut OrderBook> {
        self.orderbooks.get_mut(company)
    }
}

// test company listing and perfomr basic orderbook operations using
#[test]
fn test_company_listing() {
    let mut engine = MatchingEngine::new();
    let company = Company::new(
        "PTT PUBLIC".to_string(),
        "PTT".to_string(),
        Sector::Energy,
        Market::ThaiMarket(ThaiExchange::SET),
    );
    engine.list_new_company(company.clone());
    assert_eq!(engine.orderbooks.len(), 1);
    match engine.get_company_orderbook(&company) {
        Some(order_book) => {
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
        }
        None => panic!("Company not found"),
    };
    assert_eq!(
        engine
            .get_company_orderbook(&company)
            .unwrap()
            .buy_volume()
            .unwrap(),
        dec!(70)
    );
    assert_eq!(
        engine
            .get_company_orderbook(&company)
            .unwrap()
            .sell_volume()
            .unwrap(),
        dec!(65)
    );
}
