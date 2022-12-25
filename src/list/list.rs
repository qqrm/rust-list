#![allow(warnings)]

use std::{
    borrow::{Borrow, BorrowMut},
    cell::RefCell,
    fmt::Debug,
    mem,
    ops::Deref,
};

type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug, Default)]
pub struct Node<T> {
    pub data: T,
    pub next: Link<T>,
}

#[derive(Debug, Default)]
pub struct List<T> {
    pub head: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn push(&mut self, x: T) {
        self.head = Some(Box::new(Node {
            data: x,
            next: self.head.take(),
        }));
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.data
        })
    }

    pub fn pop_back(&mut self) -> Option<T> {
        let mut node = &mut self.head;
        while node.as_ref().map(|n| n.next.is_some())? {
            node = &mut node.as_mut()?.next;
        }
        node.take().map(|last| last.data)
    }

    pub fn empty(&self) -> bool {
        self.head.is_none()
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(mut node) = cur_link {
            cur_link = mem::replace(&mut node.next, None);
        }
    }
}

#[cfg(test)]
mod list_test {

    use super::*;

    #[test]
    fn simple() {
        let q = List::<i32> { head: None };

        assert_eq!(q.empty(), true);
    }

    #[test]
    fn add_one_item() {
        let mut q = List::new();
        let x = 7;

        assert_eq!(q.empty(), true);
        q.push(x);
        assert_eq!(q.empty(), false);
        assert_eq!(q.pop().unwrap(), x);
        assert_eq!(q.empty(), true);
    }

    #[test]
    fn pop_on_empty() {
        let mut q = List::<i32>::new();

        assert_eq!(q.empty(), true);
        assert_eq!(q.pop(), None);
        assert_eq!(q.empty(), true);
    }

    #[test]
    fn several_items() {
        let mut q = List::new();
        assert_eq!(q.empty(), true);

        q.push(1);
        q.push(2);
        q.push(3);

        assert_eq!(q.empty(), false);
        assert_eq!(q.pop().unwrap(), 3);
        assert_eq!(q.pop().unwrap(), 2);
        assert_eq!(q.pop().unwrap(), 1);
        assert_eq!(q.empty(), true);
    }

    #[test]
    fn pop_push() {
        let mut q = List::new();
        q.push(1);
        q.push(2);
        q.push(3);

        assert_eq!(q.empty(), false);

        assert_eq!(q.pop().unwrap(), 3);
        assert_eq!(q.pop().unwrap(), 2);

        q.push(7);
        q.push(8);

        assert_eq!(q.pop().unwrap(), 8);
        assert_eq!(q.pop().unwrap(), 7);

        assert_eq!(q.pop().unwrap(), 1);
        assert_eq!(q.empty(), true);
    }

    #[test]
    fn peek() {
        let mut q = List::new();
        q.push(1);
        q.push(2);
        q.push(3);

        assert_eq!(q.empty(), false);
        assert_eq!(q.pop_back().unwrap(), 1);
        assert_eq!(q.pop_back().unwrap(), 2);
        assert_eq!(q.pop_back().unwrap(), 3);
        assert_eq!(q.empty(), true);
    }

    // TODO: stack recursion limit checks
    #[test]
    fn drop_test() {
        let mut q = List::new();
        q.push(1);
        q.push(2);
        q.push(3);
        drop(q);
    }
}
