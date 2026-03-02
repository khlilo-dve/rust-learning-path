use sha2::{Digest, Sha256};

#[derive(Debug)]
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

#[derive(Debug)]
struct Block {
    index: u64,
    timestamp: i64,
    transaction: Vec<Transaction>,
    previous_hash: String,
    hash: String,
}

impl Block {
    fn new(index: u64, transaction: Vec<Transaction>, previous_hash: String) -> Block {
        let mut block = Block {
            index,
            timestamp: (1700000000),
            transaction,
            previous_hash,
            hash: String::new(),
        };
        block.hash = block.calculate_hash();
        block
    }
    fn calculate_hash(&self) -> String {
        let block_data = format!("{}{}{}", self.index, self.timestamp, self.previous_hash);
        let mut hasher = Sha256::new();
        hasher.update(block_data.as_bytes());
        let result = hasher.finalize();
        format!("{:x}", result)
    }
}

fn main() {
    println!("Web3 节点启动中...");
    let tx1 = Transaction::new(String::from("Alice"), String::from("Bob"), 50);
    let tx2 = Transaction::new(String::from("Bob"), String::from("Dave"), 20);
    let tx3 = Transaction::new(String::from("Dave"), String::from("Eve"), 100);
    let tx_pool = vec![tx1, tx2, tx3];
    let genesis_block = Block::new(0, tx_pool, String::from("0"));
    println!("成功挖出创世区块：{:#?}", genesis_block);
}
