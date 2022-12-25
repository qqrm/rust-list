use std::rc::Rc;

pub struct List<T> {
    pub head: Link<T>,
}

type Link<T> = Option<Rc<Node<T>>>;

pub struct Node<T> {
    pub data: T,
    pub next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn prepend(&self, data: T) -> List<T> {
        List {
            head: Some(Rc::new(Node {
                data: data,
                next: self.head.clone(),
            })),
        }
    }

    pub fn tail(&self) -> Self {
        List {
            head: self.head.as_ref().and_then(|node| node.next.clone()),
        }
    }

    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.data)
    }

    pub fn empty(&self) -> bool {
        self.head.is_none()
    }
}

#[cfg(test)]
mod persistant_list {
    use super::*;

    #[test]
    fn create_empty_list() {
        let list = List::<i32>::new();
        assert_eq!(list.empty(), true);
    }

    #[test]
    fn basics() {
        let list = List::new();
        assert_eq!(list.head(), None);

        let list = list.prepend(1).prepend(2).prepend(3);
        assert_eq!(list.head(), Some(&3));

        let list = list.tail();
        assert_eq!(list.head(), Some(&2));

        let list = list.tail();
        assert_eq!(list.head(), Some(&1));

        let list = list.tail();
        assert_eq!(list.head(), None);

        let list = list.tail();
        assert_eq!(list.head(), None);
    }
}
