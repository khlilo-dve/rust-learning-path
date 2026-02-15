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

struct Blockchain{
    ledger:Vec<Transaction>,
}

impl Blockchain{
    fn new()->Blockchain{
        Blockchain { 
            ledger: Vec::new(),
        }
    }
    fn
}
fn main() {

}
