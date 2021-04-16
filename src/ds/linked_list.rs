/* A node in the linked list, This struct is generic over the type of
 * the linked list
 */ 

type NodeT<T> = Option<Box<Node<T>>>; 

struct Node<T>{
    value : T,
    next : NodeT<T>
}

struct LinkedList<T>{
    head : NodeT<T>,
    len : usize
}

impl<T> LinkedList<T>
where
{
    fn new() -> LinkedList<T>{
        LinkedList{
            head: None, 
            len: 0
        }
    }

    // Method that inserts a value into the linked list
    fn insert(&mut self, value: T){
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

    //Removes first occurence of value from the list
    //Returns false if item is not in the list
    fn remove(&mut self, value: T) -> bool where T: PartialEq{
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


    //Returns a None on invalid
    fn get(&mut self, index : usize) -> Option<&T>{
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
    fn test_insert(){
        let mut list = LinkedList::new();
        list.insert(4);
        assert_eq!(list.head.as_ref().unwrap().value, 4);
    }

    #[test]
    fn test_remove(){
        let mut list = LinkedList::new();
        list.insert(1);
        list.insert(2);
        list.insert(3);
        list.insert(4);
        list.insert(5);
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
        list.insert(4);
        assert_eq!(list.len, 1);
        list.insert(4);
        assert_eq!(list.len, 2);
        list.insert(4);
        assert_eq!(list.len, 3);
    }

    #[test]
    fn test_get(){
        let mut list = LinkedList::new();
        assert!(list.get(0).is_none());
        assert!(list.get(4).is_none());
        list.insert(4);
        assert_eq!(*list.get(0).unwrap(), 4);
        list.insert(5);
        assert_eq!(*list.get(1).unwrap(), 5);
        list.insert(6);
        assert_eq!(*list.get(2).unwrap(), 6);
        assert!(list.get(3).is_none());
    }
}
