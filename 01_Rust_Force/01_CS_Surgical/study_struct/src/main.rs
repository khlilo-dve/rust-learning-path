struct Transaction {
    from: String,
    to: String,
    amount: u64,
}

fn main() {
    let tx1 = Transaction {
        from: String::from("Alice"),
        to: String::from("Bob"),
        amount: 100,
    };
    println!("金额:{}", tx1.amount);
    println!("收款人:{}", tx1.to);
}
