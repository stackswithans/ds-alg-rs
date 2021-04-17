//A queue is a FIFO (First-In, First-Out) data structure in which the element
//that is inserted first is the first one to be taken out. The elements in a queue are added at one end
//called the REAR and removed from the other end called the FRONT .

use std::cell::RefCell;
use std::rc::Rc;

type Container<T> = Option<Rc<RefCell<Node<T>>>>;

struct Node<T> {
    value: T,
    next: Container<T>,
}

pub struct Queue<T> {
    front: Container<T>,
    rear: Container<T>,
    len: usize,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Queue {
            front: None,
            rear: None,
            len: 0,
        }
    }

    //Inserts an element into a queue. The new element is added as the
    //last element of the queue.
    pub fn enqueue(&mut self, value: T) {
        let node = Node { value, next: None };
        let container = Rc::new(RefCell::new(node));
        if self.front.is_none() {
            self.front = Some(Rc::clone(&container));
            self.rear = Some(Rc::clone(&container));
        } else {
            {
                let mut rear_ref = self.rear.as_ref().unwrap().borrow_mut();
                rear_ref.next = Some(Rc::clone(&container));
            }
            self.rear = Some(Rc::clone(&container));
        }
        self.len += 1;
    }
}

#[cfg(test)]
mod test {
    use super::Queue;

    #[test]
    fn test_enqueue() {
        let mut queue = Queue::new();
        queue.enqueue(1);
        assert_eq!(
            queue.front.as_mut().unwrap().borrow().value,
            queue.rear.as_mut().unwrap().borrow().value
        );
        assert_eq!(queue.front.as_mut().unwrap().borrow().value, 1);
        queue.enqueue(2);
        assert_eq!(queue.front.as_mut().unwrap().borrow().value, 1);
        assert_eq!(queue.rear.as_mut().unwrap().borrow().value, 2);
    }
}
