struct Node{
    value:u64,
    next:Option<Box<Node>>,
}

struct LinkedList{
    head:Option<Box<Node>>,
}

impl LinkedList{
    fn new()->Self{
        LinkedList{
            head:None,
        }
    }
    fn push(&mut self,value:u64){
        let new_node=Box::new(Node{
            value:value,
            next:self.head.take()
        });
        self.head=Some(new_node);
    }
    fn pop(&mut self)->Option<u64>{
        let old_head=self.head.take();
        match old_head{
            Some(node)=>{
                self.head=node.next;
                Some(node.value)
            }
            None=>None
        }
    }
}

fn main(){
    let mut list=LinkedList::new();
    list.push(100);
    list.push(200);
    println!("成功将100和200压入链表!");
    match list.pop(){
        Some(val)=>println!("弹出{}",val),
        None=>println!("链表已经是空的了"),
    }
    match list.pop() {
        Some(val) => println!("成功弹出了：{}", val),
        None => println!("链表已经是空的了！"),
    }
    match list.pop() {
        Some(val) => println!("成功弹出了：{}", val),
        None => println!("链表已经是空的了！"),
    }
}
