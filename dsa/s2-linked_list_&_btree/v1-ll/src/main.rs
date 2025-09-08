use std::cmp::PartialOrd;

#[derive(Debug)]
pub struct LinkedList<T>(Option<(T, Box<LinkedList<T>>)>);

impl<T: PartialOrd> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList(None)
    }

    pub fn push_front(&mut self, data: T) {
        let t = self.0.take();
        self.0 = Some((data, Box::new(LinkedList(t))));
    }

    pub fn push_back(&mut self, data: T) {
        match self.0 {
            Some((_, ref mut child)) => child.push_back(data),
            None => self.push_front(data),
        }
    }

    pub fn insert_sorted(&mut self, data: T) {
        match self.0 {
            // Empty list → insert at front
            None => self.push_front(data),

            Some((ref head_data, ref mut next)) => {
                // If new data should be inserted before the head → push front
                if data < *head_data {
                    self.push_front(data);
                } else {
                    // Else, recursively insert into the child node
                    next.insert_sorted(data);
                }
            }
        }
    }
}

fn main() {
    let mut ll = LinkedList::new();
    ll.push_front(3);
    ll.push_back(12);
    ll.push_front(1);

    println!("{:?}", ll);

    let mut ll_insert_sorted = LinkedList::new();
    ll_insert_sorted.insert_sorted(3);
    ll_insert_sorted.insert_sorted(12);
    ll_insert_sorted.insert_sorted(1);
    ll_insert_sorted.insert_sorted(7);
    ll_insert_sorted.insert_sorted(5);

    println!("ll = {:?}", ll_insert_sorted);
}
