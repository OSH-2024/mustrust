use alloc::boxed::*;
use core::option::*;

// 单链表节点
struct Node<T> {
    val : T,
    next: Option<Box<Node<T>>>,// 下一个节点的指针的Option
}

// 单链表
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

    pub fn init(&mut self, data: &mut T) {
        self.head = Some(Box::new(Node::<T>{
            val : data,
            next: None,
        }));
        self.size = 1;
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

    pub fn push_pos(&mut self, data: T, pos: usize) -> bool {
        let mut p: Option<Box<Node<T>>> = self.head;
        let mut prev_p: Option<Box<Node<T>>> = None;
        for _ in 0..pos {
            if p.is_none() {
                return false;
            }
            let next_p = p.unwrap().next;
            prev_p = p.take();
            p = next_p.take();
        }
        let node = Box::new(Node::<T>{
            val : data,
            next: p,
        });
        prev_p.unwrap().next = Some(node);
        self.size += 1;
        true
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

    pub fn pop_pos(&mut self, pos: usize) -> Option<T> {
        let mut p: Option<Box<Node<T>>> = self.head;
        let mut prev_p: Option<Box<Node<T>>> = None;
        for _ in 0..pos {
            if p.is_none() {
                return None
            }
            let next_p = p.unwrap().next;
            prev_p = p.take();
            p = next_p.take();
        }
        if prev_p.is_none() {
            self.head = p.unwrap().next.take();
        }
        else {
            prev_p.unwrap().next = p.unwrap().next.take();
        }
        self.size -= 1;
        Some(p.unwrap().val)
    }

    pub fn find_data(&mut self, data: &T) -> usize {
        let mut p: Option<Box<Node<T>>> = self.head;
        let mut pos = 0;
        while p.is_some() {
            if p.unwrap().val == *data {
                return pos
            }
            p = p.unwrap().next;
            pos += 1;
        }
        -1
    }

    pub fn move_to_peek(&mut self, data: &T) -> bool {
        let pos: usize = self.find_data(data);
        if pos == -1 {
            return false
        }
        self.pop_pos(pos);
        self.push(*data);
        true
    }

    // 返回链表头节点的引用，可以改变链表头节点的val
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        if self.is_empty() {
            return None
        }
        Some(&mut self.head.as_mut().unwrap().val)
    }

    pub fn last_mut(&mut self) -> Option<&mut T> {
        if self.is_empty() {
            return None
        }
        let mut p: Option<Box<Node<T>>> = self.head;
        while p.unwrap().next.is_some() {
            p = p.unwrap().next;
        }
        Some(&mut p.unwrap().val)
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
