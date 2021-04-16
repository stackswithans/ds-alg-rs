// A linked list, in simple terms, is a linear collection of data elements. These data elements are
// called nodes. A Linked list is a data structure which in turn can be used to implement other data
// structures. Thus, it acts as a building block to implement data structures such as stacks, queues,
// and their variations. A linked list can be perceived as a train or a sequence of nodes in which each
// node contains one or more data fields and a pointer to the next node.
// But unlike an array, a linked list does not store its nodes in consecutive memory locations.
type NodeT<T> = Option<Box<Node<T>>>; 

struct Node<T>{
    value : T,
    next : NodeT<T>
}

pub struct LinkedList<T>{
    head : NodeT<T>,
    pub len : usize
}

impl<T> LinkedList<T>
where
{
    pub fn new() -> LinkedList<T>{
        LinkedList{
            head: None, 
            len: 0
        }
    }

    // Method that appends a value to the end of the list
    pub fn append(&mut self, value: T){
        let new_node = Box::new(Node{
            value,
            next: None
        });

        if self.head.is_none(){
            self.head = Some(new_node);
        }
        else{
            let mut current = self.head.as_mut();
            let mut tail = loop {
                let unwrapped = current.unwrap();
                if unwrapped.next.is_none(){
                    break unwrapped;
                }
                current = unwrapped.next.as_mut();
            };
            tail.next = Some(new_node);
        }
        self.len += 1;
    }
    // Inserts a value at the specified index
    pub fn insert(&mut self, value: T, index: usize){
        if index > self.len - 1{
            return; 
        }
        let mut new_node = Box::new(Node{
            value,
            next: None
        });

        if index == 0 {
            new_node.next = (&mut self.head).take();
            self.head = Some(new_node);
        }
        else{
            let mut i = 1;
            let mut current = &mut self.head.as_mut().unwrap().next;
            while i < self.len {
                if i == index - 1 {
                    let current_node = current.as_mut().unwrap(); 
                    new_node.next = current_node.next.take(); 
                    current_node.next = Some(new_node);
                    break;
                }
                current = &mut current.as_mut().unwrap().next;
                i += 1;
            }
        }
        self.len += 1;
    }


    //Removes first occurence of value from the list
    //Returns false if item is not in the list
    pub fn remove(&mut self, value: T) -> bool where T: PartialEq{
        if self.head.is_none(){
            return false;
        }
        else if self.head.as_ref().unwrap().value == value{
            //Claim ownership of box so that memory can be deallocated
            let head : Box<Node<T>> = self.head.take().unwrap();
            self.head = head.next;
            self.len -= 1;
            return true;
        }
        let mut node = &mut self.head;
        while node.is_some(){
            let unwrapped = node.as_mut().unwrap();
            let next_node = &mut unwrapped.next;
            if next_node.as_ref().unwrap().value == value{
                let next_node : Box<Node<T>> = next_node.take().unwrap();
                unwrapped.next = next_node.next;
                self.len -= 1;
                return true;
            }
            node = &mut unwrapped.next;
        }
        false
    }


    //Gets a reference to an element in the array
    pub fn get(&mut self, index : usize) -> Option<&T>{
        if self.len as isize - 1 < index as isize{
           return None;
        }
        let mut counter = 0;
        let mut node = self.head.as_mut();
        loop{
            if counter == self.len{
                return None;
            }
            else if counter == index{
                return Some(&node.unwrap().value);
            }
            node = node.unwrap().next.as_mut();
            counter += 1;
        };
    }
}


#[cfg(test)]
mod tests{
    use super::{LinkedList};
    #[test]
    fn test_append(){
        let mut list = LinkedList::new();
        list.append(4);
        assert_eq!(list.head.as_ref().unwrap().value, 4);
    }

    #[test]
    fn test_insert_at_head(){
        let mut list = LinkedList::new();
        list.append(2);
        list.append(3);
        list.append(4);
        list.append(5);
        list.insert(1, 0);
        assert_eq!(list.len, 5);
        assert_eq!(*list.get(0).unwrap(), 1);
        assert_eq!(*list.get(1).unwrap(), 2);
    }

    #[test]
    fn test_insert_at_between(){
        let mut list = LinkedList::new();
        list.append(2);
        list.append(3);
        list.append(4);
        list.append(5);
        list.insert(10, 2);
        assert_eq!(list.len, 5);
        assert_eq!(*list.get(1).unwrap(), 3);
        assert_eq!(*list.get(2).unwrap(), 10);
        assert_eq!(*list.get(3).unwrap(), 4);
    }

    #[test]
    fn test_insert_at_tail(){
        let mut list = LinkedList::new();
        list.append(2);
        list.append(3);
        list.append(4);
        list.append(5);
        list.insert(8,3);
        assert_eq!(list.len, 5);
        assert_eq!(*list.get(3).unwrap(), 8);
        assert_eq!(*list.get(4).unwrap(), 5);
    }

    #[test]
    fn test_remove(){
        let mut list = LinkedList::new();
        list.append(1);
        list.append(2);
        list.append(3);
        list.append(4);
        list.append(5);
        assert_eq!(list.len, 5);
        assert!(list.remove(1));
        assert_eq!(list.len, 4);
        assert_eq!(*list.get(0).unwrap(), 2);

        assert!(list.remove(3));
        assert_eq!(list.len, 3);
        assert_eq!(*list.get(0).unwrap(), 2);
        assert_eq!(*list.get(1).unwrap(), 4);

        assert!(list.remove(5));
        assert_eq!(list.len, 2);
        assert_eq!(*list.get(0).unwrap(), 2);
        assert_eq!(*list.get(1).unwrap(), 4);
    }

    #[test]
    fn test_len(){
        let mut list = LinkedList::new();
        assert_eq!(list.len, 0);
        list.append(4);
        assert_eq!(list.len, 1);
        list.append(4);
        assert_eq!(list.len, 2);
        list.append(4);
        assert_eq!(list.len, 3);
    }

    #[test]
    fn test_get(){
        let mut list = LinkedList::new();
        assert!(list.get(0).is_none());
        assert!(list.get(4).is_none());
        list.append(4);
        assert_eq!(*list.get(0).unwrap(), 4);
        list.append(5);
        assert_eq!(*list.get(1).unwrap(), 5);
        list.append(6);
        assert_eq!(*list.get(2).unwrap(), 6);
        assert!(list.get(3).is_none());
    }
}
