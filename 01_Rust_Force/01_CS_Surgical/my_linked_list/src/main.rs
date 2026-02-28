struct Node{
    value:u64,
    next:Option<Box<Node>>,
}

struct LinkedList{
    head:Option<Box<Node>>,
}

fn main(){
    println!("链表图纸编译通过");
}