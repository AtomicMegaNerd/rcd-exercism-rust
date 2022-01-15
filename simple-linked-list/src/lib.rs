use std::iter::FromIterator;
use std::mem;

pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
}

struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Default for SimpleLinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList { head: None }
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn len(&self) -> usize {
        let mut count = 0;
        let mut node = &self.head;
        while let Some(n) = node {
            count += 1;
            node = &n.next;
        }
        count
    }

    pub fn push(&mut self, _element: T) {
        let new_node = Some(Box::new(Node {
            data: _element,
            next: mem::replace(&mut self.head, None),
        }));
        self.head = new_node;
    }

    pub fn pop(&mut self) -> Option<T> {
        if let Some(node) = mem::replace(&mut self.head, None) {
            self.head = node.next;
            Some(node.data)
        } else {
            None
        }
    }

    pub fn peek(&self) -> Option<&T> {
        if let Some(node) = &self.head {
            Some(&node.data)
        } else {
            None
        }
    }

    pub fn rev(self) -> SimpleLinkedList<T> {
        let mut new_list = SimpleLinkedList::new();
        let mut node = self.head;
        while let Some(n) = node {
            new_list.push(n.data);
            node = n.next;
        }
        new_list
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(_iter: I) -> Self {
        let mut list = SimpleLinkedList::new();
        _iter.into_iter().for_each(|item| list.push(item));
        list
    }
}

impl<T> From<SimpleLinkedList<T>> for Vec<T> {
    fn from(mut _linked_list: SimpleLinkedList<T>) -> Vec<T> {
        let mut vals = Vec::<T>::new();
        let mut node = _linked_list.rev().head;
        while let Some(n) = node {
            vals.push(n.data);
            node = n.next;
        }
        vals
    }
}
