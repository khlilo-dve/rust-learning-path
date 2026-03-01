struct Node {
    value: u64,
    next: Option<Box<Node>>,
}

struct LinkedList {
    head: Option<Box<Node>>,
}

impl LinkedList {
    fn new() -> Self {
        LinkedList { head: None }
    }
    fn push(&mut self, value: u64) {
        let new_node = Box::new(Node {
            value: value,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }
    fn pop(&mut self) -> Option<u64> {
        let old_head = self.head.take();
        match old_head {
            Some(node) => {
                self.head = node.next;
                Some(node.value)
            }
            None => None,
        }
    }
}

impl Drop for LinkedList {
    fn drop(&mut self) {
        while let Some(_) = self.pop() {}
    }
}

fn main() {
    let mut list = LinkedList::new();
    println!("开始装填100000枚核弹头...");
    (0..100000).for_each(|i| list.push(i));
    println!("装填完毕！准备启动Dorp拆弹程序");
}
