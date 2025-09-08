// Immutable outside, but can mutate interior
use std::cell::RefCell;
// Reference Counting pointer
use std::rc::{Rc, Weak};

#[allow(dead_code)]
#[derive(Debug)]
pub struct Node<T> {
    data: T,
    next: Option<Rc<RefCell<Node<T>>>>,
    prev: Option<Weak<RefCell<Node<T>>>>,
}

#[derive(Debug)]
pub struct DoublyLinkedList<T> {
    first: Option<Rc<RefCell<Node<T>>>>,
    last: Option<Weak<RefCell<Node<T>>>>,
}

impl<T> DoublyLinkedList<T> {
    pub fn new() -> Self {
        DoublyLinkedList {
            first: None,
            last: None,
        }
    }

    pub fn push_front(&mut self, data: T) {
        match self.first.take() {
            Some(r) => {
                // Create new front object
                let new_front = Rc::new(RefCell::new(Node {
                    data,
                    next: Some(r.clone()),
                    prev: None,
                }));
                // Tell the first object this is now in front of it
                let mut m = r.borrow_mut();
                m.prev = Some(Rc::downgrade(&new_front));
                // Put this on the front
                self.first = Some(new_front);
            }
            None => {
                let new_data = Rc::new(RefCell::new(Node {
                    data,
                    next: None,
                    prev: None,
                }));
                self.last = Some(Rc::downgrade(&new_data));
                self.first = Some(new_data);
            }
        }
    }

    pub fn push_back(&mut self, data: T) {
        match self.last.take() {
            Some(r) => {
                // Create new front object
                let new_back = Rc::new(RefCell::new(Node {
                    data,
                    prev: Some(r.clone()),
                    next: None,
                }));
                // Tell the last object this is now behind of it
                let st = Weak::upgrade(&r).unwrap();
                let mut m = st.borrow_mut();
                m.next = Some(new_back.clone());
                // Put this on the back
                self.last = Some(Rc::downgrade(&new_back));
            }
            None => {
                let new_data = Rc::new(RefCell::new(Node {
                    data,
                    next: None,
                    prev: None,
                }));
                self.last = Some(Rc::downgrade(&new_data));
                self.first = Some(new_data);
            }
        }
    }
}

fn main() {
    let mut dll: DoublyLinkedList<_> = DoublyLinkedList::new();
    dll.push_front(6);
    dll.push_back(11);
    dll.push_front(5);
    dll.push_back(15);
    dll.push_front(4);
    println!("dll = {:?}", dll);
}
