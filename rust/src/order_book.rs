use std::collections::HashMap;

use uuid::Uuid;

use crate::ticker::Ticker;

pub struct OrderBook {
    tickers: HashMap<u32, Ticker>,
}

pub enum OrderBookError {
    LimitOrderNotFound { id: Uuid },
}

impl OrderBook {
    pub fn new() -> Self {
        OrderBook {
            tickers: HashMap::new(),
        }
    }

    ///! Execute a market order
    ///! Will attempt to match all quantities of the market order
    ///! until the entire order is filled. If it cannot be filled then
    ///! will return how many were filled.
    pub fn execute_market_order(&mut self, ticker: u32, quantity: u32) -> Option<u32> {
        todo!()
    }

    ///! Place a limit order
    ///! Will attempt to match all quantities of the limit order at the
    ///! requested price until the entire order is filled. If it cannot
    ///! be filled then will add the rest of the order to the order book.
    pub fn place_limit_order(&mut self, ticker: u32, quantity: u32, price: u32) -> Uuid {
        todo!()
    }

    ///! Cancel a limit order
    ///! Will attempt to cancel a limit order with specified id, if order does
    ///! not exist then will return `OrderBookError::LimitOrderNotFound{id}`.
    pub fn cancel_limit_order(&mut self, id: Uuid) -> Result<(), OrderBookError> {
        return Err(OrderBookError::LimitOrderNotFound { id });
    }
}
