use std::collections::BinaryHeap;

use crate::price_level::PriceLevel;

pub struct Ticker {
    id: u32,
    asks: BinaryHeap<PriceLevel>,
    bids: BinaryHeap<PriceLevel>,
}

impl Ticker {
    pub fn new(id: u32) -> Self {
        Ticker {
            id,
            asks: BinaryHeap::new(),
            bids: BinaryHeap::new(),
        }
    }
}

impl PartialEq for Ticker {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
