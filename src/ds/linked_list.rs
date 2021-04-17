// A linked list, in simple terms, is a linear collection of data elements. These data elements are
// called nodes. A Linked list is a data structure which in turn can be used to implement other data
// structures. Thus, it acts as a building block to implement data structures such as stacks, queues,
// and their variations. A linked list can be perceived as a train or a sequence of nodes in which each
// node contains one or more data fields and a pointer to the next node.
// But unlike an array, a linked list does not store its nodes in consecutive memory locations.
pub(super) type NodeT<T> = Option<Box<Node<T>>>;

pub(super) struct Node<T> {
    value: T,
    next: NodeT<T>,
}

pub struct LinkedList<T> {
    head: NodeT<T>,
    len: usize,
}

impl<T> LinkedList<T> {
    pub fn new() -> LinkedList<T> {
        LinkedList { head: None, len: 0 }
    }

    //Returns a reference to the element at the specified position.
    //Panics if index is large than list.len()
    pub fn get(&self, index: usize) -> &T {
        if index >= self.len {
            panic!("Index is larger than the length of the list")
        }
        let mut i = 0;
        let mut node = &self.head;
        while node.is_some() {
            if i == index {
                break;
            }
            node = &node.as_ref().unwrap().next;
            i += 1;
        }
        &(node.as_ref().unwrap().value)
    }

    //Returns the length of the list
    pub fn len(&self) -> usize {
        self.len
    }

    //Inserts a value at the given position.
    //If index is larger than list.len(), value is appended
    //to the end of the list.
    pub fn insert(&mut self, value: T, index: usize) {
        let mut new_node = Box::new(Node { value, next: None });
        if index == 0 {
            new_node.next = (&mut self.head).take();
            self.head = Some(new_node);
        } else {
            let mut i = 0;
            let mut current = &mut self.head;
            while current.as_mut().unwrap().next.is_some() && i < index - 1 {
                current = &mut current.as_mut().unwrap().next;
                i += 1;
            }
            //Node is inserted before the end
            let next = &mut current.as_mut().unwrap().next;
            new_node.next = next.take();
            current.as_mut().unwrap().next = Some(new_node);
        }
        self.len += 1;
    }

    //Adds value to the end of the list
    pub fn append(&mut self, value: T) {
        self.insert(value, self.len);
    }

    //Removes value at index from the list and returns it
    pub fn remove(&mut self, index: usize) -> T {
        if index >= self.len {
            panic!("Index is larger than the length of the list");
        }
        if index == 0 {
            let head = self.head.take().unwrap();
            self.head = head.next;
            self.len -= 1;
            head.value
        } else {
            let mut i = 0;
            let mut current = &mut self.head;
            while i < index - 1 {
                i += 1;
                current = &mut current.as_mut().unwrap().next;
            }
            let next = current.as_mut().unwrap().next.take().unwrap();
            current.as_mut().unwrap().next = next.next;
            self.len -= 1;
            next.value
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LinkedList;
    #[test]
    fn test_len() {
        let mut list = LinkedList::new();
        assert_eq!(list.len(), 0);
        list.append(4);
        assert_eq!(list.len(), 1);
        list.append(4);
        assert_eq!(list.len(), 2);
        list.append(4);
        assert_eq!(list.len(), 3);
    }

    #[test]
    #[should_panic]
    fn test_get_panic() {
        let mut list = LinkedList::<i32>::new();
        list.get(0);
    }

    #[test]
    fn test_get() {
        let mut list = LinkedList::new();
        list.append(4);
        assert_eq!(*list.get(0), 4);
        list.append(5);
        assert_eq!(*list.get(1), 5);
        list.append(6);
        assert_eq!(*list.get(2), 6);
    }

    #[test]
    fn test_append() {
        let mut list = LinkedList::new();
        list.append(4);
        assert_eq!(list.head.as_ref().unwrap().value, 4);
    }

    #[test]
    fn test_insert_at_head() {
        let mut list = LinkedList::new();
        list.append(2);
        list.append(3);
        list.append(4);
        list.append(5);
        list.insert(1, 0);
        assert_eq!(list.len, 5);
        assert_eq!(*list.get(0), 1);
        assert_eq!(*list.get(1), 2);
    }

    #[test]
    fn test_insert_between() {
        let mut list = LinkedList::new();
        list.append(2);
        list.append(3);
        list.append(4);
        list.append(5);
        list.insert(10, 2);
        assert_eq!(list.len(), 5);
        assert_eq!(*list.get(1), 3);
        assert_eq!(*list.get(2), 10);
        assert_eq!(*list.get(3), 4);
    }

    #[test]
    fn test_insert_at_tail() {
        let mut list = LinkedList::new();
        list.append(2);
        list.append(3);
        list.append(4);
        assert_eq!(list.len(), 3);
        assert_eq!(*list.get(2), 4);
        list.insert(8, list.len());
        assert_eq!(*list.get(3), 8);
    }

    #[test]
    #[should_panic(expected = "Index is larger than the length of the list")]
    fn test_remove_panic() {
        let mut list = LinkedList::<i32>::new();
        list.remove(1);
    }

    #[test]
    fn test_remove_at_head() {
        let mut list = LinkedList::new();
        list.append(1);
        list.append(2);
        assert_eq!(list.len(), 2);
        assert_eq!(list.remove(0), 1);
        assert_eq!(list.len, 1);
        assert_eq!(list.remove(0), 2);
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn test_remove_between() {
        let mut list = LinkedList::new();
        list.append(1);
        list.append(2);
        list.append(3);
        assert_eq!(list.len(), 3);
        assert_eq!(list.remove(1), 2);
        assert_eq!(list.len(), 2);
        assert_eq!(*list.get(0), 1);
        assert_eq!(*list.get(1), 3);
    }

    #[test]
    fn test_remove_at_tail() {
        let mut list = LinkedList::new();
        list.append(1);
        list.append(4);
        list.append(5);
        assert_eq!(list.len(), 3);
        assert_eq!(list.remove(2), 5);
        assert_eq!(*list.get(1), 4);
    }
}
