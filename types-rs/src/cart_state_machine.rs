// State Machines!

mod shopping {
    struct Product { id: i32, name: String };

    struct ActiveCart { UnpaidItems: Vec<Product> }

    struct PaidCart { PaidItems: Vec<Product>, Payment: i64 }

    enum ShoppingCart {
        Empty,
        Active(ActiveCart),
        Paid(PaidCart),
    }
}

