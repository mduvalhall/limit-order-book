use uuid::Uuid;

pub enum Order {
    Ask { id: Uuid, quantity: u32, price: u32 },
    Bid { id: Uuid, quantity: u32, price: u32 },
}

impl Order {
    pub fn new_ask(quantity: u32, price: u32) -> Self {
        Order::Ask {
            id: Uuid::new_v4(),
            quantity,
            price,
        }
    }

    pub fn new_bid(quantity: u32, price: u32) -> Self {
        Order::Bid {
            id: Uuid::new_v4(),
            quantity,
            price,
        }
    }
}
