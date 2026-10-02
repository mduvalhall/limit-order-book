use std::collections::VecDeque;

use crate::order::Order;

pub struct PriceLevel {
    price: u32,
    orders: VecDeque<Order>,
}

impl PriceLevel {
    pub fn new(price: u32) -> Self {
        PriceLevel {
            price,
            orders: VecDeque::new(),
        }
    }
}
