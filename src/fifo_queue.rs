#![allow(warnings)]

use std::{
    borrow::{Borrow, BorrowMut},
    cell::RefCell,
    ops::Deref,
};

#[derive(Debug, Default)]
struct Node<T> {
    pub data: T,
    pub next: Option<Box<Self>>,
}

#[derive(Debug, Default)]
struct FiloQueue<T> {
    pub root: Option<Box<Node<T>>>,
}

impl<T> FiloQueue<T> {
    fn new() -> Self {
        Self { root: None }
    }

    fn push(&mut self, x: T) {
        self.root = Some(Box::new(Node {
            data: x,
            next: self.root.take(),
        }));
    }

    fn pop(&mut self) -> Option<T> {
        self.root.take().map(|node| {
            self.root = node.next;
            node.data
        })
    }

    fn peek(&self) -> Option<T> {
        unimplemented!()
    }

    fn empty(&self) -> bool {
        self.root.is_none()
    }
}

mod test {

    use super::*;

    #[test]
    fn simple() {
        let q = FiloQueue::<i32> { root: None };

        assert_eq!(q.empty(), true);
    }

    #[test]
    fn add_one_item() {
        let mut q = FiloQueue::new();
        let x = 7;

        assert_eq!(q.empty(), true);
        q.push(x);
        assert_eq!(q.empty(), false);
        assert_eq!(q.pop().unwrap(), x);
        assert_eq!(q.empty(), true);
    }

    #[test]
    fn add_two_items() {
        let mut q = FiloQueue::new();
        q.push(1);
        dbg!(&q);
        q.push(2);
        dbg!(&q);
        q.push(3);
        dbg!(&q);

        assert_eq!(q.empty(), false);
        assert_eq!(q.pop().unwrap(), 3);
        assert_eq!(q.pop().unwrap(), 2);
        assert_eq!(q.pop().unwrap(), 1);
        assert_eq!(q.empty(), true);
    }

    // #[bench]
    // fn add_two_items() {
    //     let mut q = FiloQueue::new();
    //     q.push(1);
    //     dbg!(&q);
    //     q.push(2);
    //     dbg!(&q);
    //     q.push(3);
    //     dbg!(&q);

    //     assert_eq!(q.empty(), false);
    //     assert_eq!(q.pop(), 3);
    //     assert_eq!(q.pop(), 2);
    //     assert_eq!(q.pop(), 1);
    //     assert_eq!(q.empty(), true);
    // }
}
