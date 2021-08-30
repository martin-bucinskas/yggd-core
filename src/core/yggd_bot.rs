use tokio::time::error::Error;
use alpaca_finance::{Alpaca, Streamer, StreamMessage, Order, OrderType, TimeInForce};

pub struct YggdBot {
    client: Alpaca
}

pub trait Market {
    async fn place_sell_order(&self, symbol: Box<str>, qty: u32, order_type: OrderType, time_in_force: TimeInForce, limit_price: f64) -> Result<Order, Error>;
    async fn place_buy_order(&self, symbol: Box<str>, qty: u32, order_type: OrderType, time_in_force: TimeInForce, limit_price: f64) -> Result<Order, Error>;
}

impl YggdBot {
    pub fn new(&mut self) -> Self {
        let key_id: &'static str = env!("KEY_ID");
        let key_secret: &'static str = env!("KEY_SECRET");

        YggdBot {
            client: Alpaca::paper(key_id, key_secret).await.unwrap()
        }
    }

    pub async fn start(&mut self) -> Result<(), Error> {
        Result::Ok(())
    }

    pub async fn init_stream(&mut self) -> Result<(), Error> {
        let streamer = Streamer::new(&self.client);
        streamer.start().await
            .for_each(|msg| {
                match msg {
                    StreamMessage::Account(_) => println!("Account update: {}", msg),
                    StreamMessage::Order(_) => println!("Order update: {}", msg),
                    _ => println!("Unexpected message: {}", msg)
                }
                future::ready(())
            })
            .await;

        Result::Ok(())
    }
}

impl Market for YggdBot {
    async fn place_sell_order(&self, symbol: Box<str>, qty: u32, order_type: OrderType, time_in_force: TimeInForce, limit_price: f64) -> Result<Order, Error> {
        let order = Order::sell(symbol.as_ref(), qty, order_type, time_in_force)
            .limit_price(limit_price)
            .place(&self.client).await.unwrap();
        Result::Ok(order)
    }

    async fn place_buy_order(&self, symbol: Box<str>, qty: u32, order_type: OrderType, time_in_force: TimeInForce, limit_price: f64) -> Result<Order, Error> {
        let order = Order::sell(symbol.as_ref(), qty, order_type, time_in_force)
            .limit_price(limit_price)
            .place(&self.client).await.unwrap();
        Result::Ok(order)
    }
}