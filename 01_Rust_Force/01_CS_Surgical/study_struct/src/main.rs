struct Transaction {
    from: String,
    to: String,
    amount: u64,
}

impl Transaction {
    fn new(from: String, to: String, amount: u64) -> Transaction {
        Transaction {
            from: from,
            to: to,
            amount: amount,
        }
    }
    fn print_receipt(&self) {
        println!(
            "交易回执:{}->{}|数额:{}BTC",
            self.from, self.to, self.amount
        );
    }
    fn change_amount(&mut self, new_amount: u64) {
        self.amount = new_amount;
        println!("金额已修改为:{}", self.amount);
    }
}

struct Blockchain {
    ledger: Vec<Transaction>,
}

impl Blockchain {
    fn new() -> Blockchain {
        Blockchain { ledger: Vec::new() }
    }
    fn add_transaction(&mut self, tx: Transaction) {
        self.ledger.push(tx);
        println!("系统成功录入一笔交易");
    }
    fn print_chain(&self) {
        println!("\n区块链账本数据");
        for item in &self.ledger {
            item.print_receipt();
        }
        println!("共计{}笔数据\n", self.ledger.len());
    }
}
fn main() {
    print!("区块链系统启动中");
    let mut chain = Blockchain::new();
    let tx1 = Transaction::new(String::from("Alice"), String::from("Bob"), 50);
    let tx2 = Transaction::new(String::from("Bob"), String::from("Dave"), 20);
    let tx3 = Transaction::new(String::from("Dave"), String::from("Eve"), 100);
    chain.add_transaction(tx1);
    chain.add_transaction(tx2);
    chain.add_transaction(tx3);
    chain.print_chain();
}
