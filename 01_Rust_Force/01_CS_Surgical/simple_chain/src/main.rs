use sha2::{Digest, Sha256};
use std::time::Instant;

#[derive(Debug, Clone)]
struct Block {
    id: u64,
    data: String,
    prev_hash: String,
    nonce: u64,
    hash: String,
}

impl Block {
    fn calculate_hash(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(self.id.to_string());
        hasher.update(&self.data);
        hasher.update(&self.prev_hash);
        hasher.update(self.nonce.to_string());
        let result = hasher.finalize();
        format!("{:x}", result)
    }

    fn mine(&mut self, difficulty: usize) {
        let target = "0".repeat(difficulty);
        loop {
            let current_hash = self.calculate_hash();
            if current_hash.starts_with(&target) {
                self.hash = current_hash;
                break;
            } else {
                self.nonce += 1;
            }
        }
    }
}

struct Blockchain {
    blocks: Vec<Block>,
}

impl Blockchain {
    fn new() -> Self {
        let mut genesis = Block {
            id: 0,
            data: String::from("Genesis Block"),
            prev_hash: String::from("0000000000000000"),
            nonce: 0,
            hash: String::new(),
        };
        genesis.mine(2);
        Blockchain {
            blocks: vec![genesis],
        }
    }

    fn add_block(&mut self, data: String) {
        let prev_block = self.blocks.last().unwrap();
        let mut new_block = Block {
            id: prev_block.id + 1,
            data: data,
            prev_hash: prev_block.hash.clone(),
            nonce: 0,
            hash: String::new(),
        };
        // 降低一点难度，方便演示，不用等太久
        new_block.mine(3);
        self.blocks.push(new_block);
    }

    // 🔥 新增：链条完整性校验 (谎言探测器)
    fn is_chain_valid(&self) -> bool {
        for i in 1..self.blocks.len() {
            let current = &self.blocks[i];
            let previous = &self.blocks[i - 1];

            // 1. 检查数据是否被篡改 (重算哈希)
            if current.hash != current.calculate_hash() {
                println!("❌ Block {} 数据哈希不匹配！数据被篡改！", current.id);
                return false;
            }

            // 2. 检查链条是否断裂 (前驱哈希对不上)
            if current.prev_hash != previous.hash {
                println!("❌ Block {} 指向错误的上一块！链条断裂！", current.id);
                return false;
            }
        }
        true
    }
}

fn main() {
    let mut chain = Blockchain::new();

    println!("--- 🛠️  正常挖矿中 ---");
    chain.add_block(String::from("Alice -> Bob 100 BTC"));
    chain.add_block(String::from("Bob -> Charlie 50 BTC"));

    // 1. 第一次检查：应该是健康的
    println!("\n🔍 状态检查 1: 链条是否有效? {}", chain.is_chain_valid());

    // ---------------------------------------------
    // 😈 黑客攻击开始！
    // ---------------------------------------------
    println!("\n--- 😈 黑客正在修改第 2 个区块的数据 ---");

    // 我们利用 Rust 的可变引用，强行修改内存里的数据
    // 注意：在真实区块链里，你改不了别人的节点，但你可以改自己本地的
    let mut block_to_hack = &mut chain.blocks[1];
    block_to_hack.data = String::from("Alice -> Hacker 10000 BTC"); // 把钱转给自己！

    println!("😈 数据已篡改: {}", block_to_hack.data);

    // 2. 第二次检查：应该报警
    println!("\n🔍 状态检查 2: 链条是否有效? {}", chain.is_chain_valid());
}
