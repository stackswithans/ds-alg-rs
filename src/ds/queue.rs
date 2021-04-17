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

    pub fn len(&self) -> usize {
        self.len
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

    //Removes the first element in the queue.
    //Panics if the queue is empty
    pub fn dequeue(&mut self) -> T {
        if self.front.is_none() {
            panic!("Cannot dequeue! The queue is empty");
        }
        if self.len == 1 {
            //Make sure that the Rc ref
            //is dropped
            self.rear = None;
        }
        let container = Rc::try_unwrap(self.front.take().unwrap()).ok().unwrap();
        let node = container.into_inner();
        self.front = node.next;
        self.len -= 1;
        node.value
    }
}

#[cfg(test)]
mod test {
    use super::Queue;

    #[test]
    fn test_len() {
        let mut queue = Queue::new();
        assert_eq!(queue.len(), 0);
        queue.enqueue(1);
        assert_eq!(queue.len(), 1);
        queue.enqueue(2);
        assert_eq!(queue.len(), 2);
        queue.dequeue();
        assert_eq!(queue.len(), 1);
        queue.dequeue();
        assert_eq!(queue.len(), 0);
        queue.enqueue(1);
        assert_eq!(queue.len(), 1);
    }

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

    #[test]
    fn test_dequeue() {
        let mut queue = Queue::new();
        queue.enqueue(1);
        queue.enqueue(2);
        queue.enqueue(3);
        assert_eq!(queue.dequeue(), 1);
        assert_eq!(queue.dequeue(), 2);
        assert_eq!(queue.dequeue(), 3);
    }

    #[test]
    #[should_panic(expected = "Cannot dequeue! The queue is empty")]
    fn test_dequeue_panics() {
        let mut queue: Queue<i32> = Queue::new();
        queue.dequeue();
    }
}
