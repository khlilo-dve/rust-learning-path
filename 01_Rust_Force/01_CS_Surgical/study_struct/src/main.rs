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

#[derive(Debug)]
struct Blockchain {
    chain: Vec<Block>,
}

impl Blockchain {
    fn new() -> Self {
        let empty_tx_pool = vec![];
        let genesis_block = Block::new(0, empty_tx_pool, String::from("0"));
        Blockchain {
            chain: vec![genesis_block],
        }
    }
    fn add_block(&mut self, transactions: Vec<Transaction>) {
        let previous_block = self.chain.last().unwrap();
        let previous_hash = previous_block.hash.clone();
        let new_index = previous_block.index + 1;
        let new_block = Block::new(new_index, transactions, previous_hash);
        self.chain.push(new_block);
    }
}

fn main() {
    println!("Web3 节点启动中...");
    let mut my_crypto_chain = Blockchain::new();
    let tx1 = Transaction::new(String::from("Alice"), String::from("Bob"), 50);
    let tx2 = Transaction::new(String::from("Bob"), String::from("Dave"), 20);
    println!("正在打包区块 #1...");
    my_crypto_chain.add_block(vec![tx1, tx2]);
    println!("当前账本状态: {:#?}", my_crypto_chain);
}
