// 单链表节点
#[derive(Debug)]
struct Node<T> {
    val : T,
    next: Option<Box<Node<T>>>,// 下一个节点的指针的Option
}

// 单链表
#[derive(Debug)]
struct Linkedlist<T> {
    head: Option<Box<Node<T>>>,
    size: usize,
}

impl<T> Linkedlist<T> {
    // 新建一个空链表
    pub fn new() -> Self {
        Linkedlist {
            head: None,
            size: 0,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn len(&self) -> usize {
        self.size
    }

    // 头插法
    pub fn push(&mut self, data: T) -> &mut Self {
        let node = Box::new(Node::<T>{
            val : data,
            next: self.head.take(),//如果不take，就会出现多个指针指向同一个节点，不符合所有权规则，可变引用可以进行take,take之后就会释放，变成None
        });
        self.head = Some(node);
        self.size += 1;
        self
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            return None
        }
        // self.head = self.head.next会导致两次可变借用
        let head = self.head.take().unwrap();
        self.head = head.next;
        self.size -= 1;
        Some(head.val)
    }

    // 返回链表头节点的引用，可以改变链表头节点的val
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        if self.is_empty() {
            return None
        }
        return Some(&mut self.head.as_mut().unwrap().val)
    }

    // 反转链表
    pub fn rev(&mut self) -> Linkedlist<T> {
        let mut rev_list = Linkedlist::<T>::new();
        while !self.is_empty() {
            rev_list.push(self.pop().unwrap());
        }
        rev_list
    }

}

impl<T: std::fmt::Debug> Linkedlist<T> {
    // 遍历链表
    pub fn traverse(&self) {
        let mut current = &self.head;
        while let Some(node) = current {
            println!("{:?}", node.val);
            current = &node.next;
        }
    }
}


fn main() {
    let mut list = Linkedlist::new();
    list.push(1).push(2).push(3);
    if let Some(value) = list.peek_mut() {
        *value = 10;
    }
    println!("the length of list is {:?}",list.len());
    list.traverse();
    let list2 = list.rev();
    println!("The reversed list: ");
    list2.traverse();
}