#[derive(Debug, PartialEq, Eq)]
enum PaymentType {
    DigitalToken,
    Cash
}

#[derive(Debug)]
struct Seller {
    payment_type: PaymentType,
    price: f32,
    balance: f32
}

#[derive(Debug)]
struct Buyer {
    name: String,
    payment_type: PaymentType,
    balance: f32
}

#[derive(Debug)]
struct BuyerGroup {
    members: Vec<Buyer>
}

impl BuyerGroup {
    fn add_member(&mut self, buyer: Buyer) {
        self.members.push(buyer);
    }

    fn find_buyer(&self, payment_type: PaymentType) -> i32 {
        let mut index = -1;
        for i in 0..self.members.len() {
            if self.members[i].payment_type == payment_type {
                index = i as i32;
                break;
            }
        }
        index
    }

    fn buy(&mut self, buyer_idx: usize, seller: &Seller) {
        let mut buyer = &mut self.members[buyer_idx];
        while buyer.balance - seller.price > 0.0 {
            buyer.balance -= seller.price;
        }
    }
}

fn main() {
    
    let john_buyer = Buyer {
        name: "John".to_owned(),
        payment_type: PaymentType::DigitalToken,
        balance: 100.00
    };

    let sally_buyer = Buyer {
        name: "Sally".to_owned(),
        payment_type: PaymentType::Cash,
        balance: 100.00
    };

    let mut buyer_group = BuyerGroup {
        members: Vec::new()
    };

    buyer_group.add_member(john_buyer);
    buyer_group.add_member(sally_buyer);

    let seller = Seller {
        payment_type: PaymentType::Cash,
        price: 10.0,
        balance: 0.0,
    };

    println!("BEFORE: {:?}", buyer_group);

    let buyer_idx: usize = buyer_group.find_buyer(PaymentType::Cash) as usize;
    if buyer_idx > 0 {
        buyer_group.buy(buyer_idx, &seller);
        println!("AFTER: {:?}", buyer_group);
    };
    


    // Check if can still use PaymentType after this
    let b2_idx = buyer_group.find_buyer(PaymentType::DigitalToken);

    let b2 = &buyer_group.members[b2_idx as usize];

    println!("{:?}", b2);

}
