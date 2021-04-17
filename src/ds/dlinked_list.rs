// A doubly linked list or a two-way linked list is a more complex type of linked list which contains
// a pointer to the next as well as the previous node in the sequence. Therefore, it consists of three
// parts—data, a pointer to the next node, and a pointer to the previous node

// Since each node will be pointed at by two other nodes,
// we will use raw pointers to avoid dealing with the hassle
// of having to use the interior mutability pattern
use std::ptr;

struct Node<T> {
    prev: *mut Node<T>,
    value: T,
    next: *mut Node<T>,
}

pub struct DoublyLinkedList<T> {
    head: *mut Node<T>,
}

impl<T> Drop for DoublyLinkedList<T> {
    fn drop(&mut self) {
        println!("Should be freeing memory");
    }
}

impl<T> DoublyLinkedList<T> {
    pub fn new() -> DoublyLinkedList<T> {
        DoublyLinkedList {
            head: ptr::null_mut(),
        }
    }

    //Returns the size of the list
    pub fn len(&self) -> usize {
        let mut i = 0;
        let mut ptr = self.head;
        while !ptr.is_null() {
            //SAFETY: Deref is okay, pointer was obtained from box
            ptr = unsafe { (*ptr).next };
            i += 1;
        }
        i
    }

    //Returns a reference to the element at the specified position.
    //If index is large than list.len(), None is returned.
    pub fn get(&self, index: usize) -> Option<&T> {
        let mut i = 0;
        let mut ptr = self.head;
        while !ptr.is_null() {
            if i == index {
                //SAFETY: Deref is okay, pointer was obtained from box
                // Pointer should not be null as the length of the list
                // guarantees that only valid pointers are accessed
                let value_ref = unsafe { &(*ptr).value };
                return Some(value_ref);
            }
            //SAFETY: Deref is okay, pointer was obtained from box
            ptr = unsafe { (*ptr).next };
            i += 1;
        }
        None
    }

    //Inserts a value at the given position.
    //If index is larger than list.len(), value is appended
    //to the end of the list.
    pub fn insert(&mut self, value: T, index: usize) {
        let new_node = Box::new(Node {
            prev: ptr::null_mut(),
            value,
            next: ptr::null_mut(),
        });
        let new_ptr = Box::into_raw(new_node);
        if index == 0 {
            if self.head.is_null() {
                self.head = new_ptr;
            } else {
                //SAFETY: Dereferencing this pointer should be okay since
                // it was obtained from a box.
                unsafe {
                    (*self.head).prev = new_ptr;
                    (*new_ptr).next = self.head;
                }
                self.head = new_ptr;
            }
        } else {
            let mut ptr = self.head;
            let mut i = 0;
            //SAFETY: Dereferencing this pointer should be okay since
            // it was obtained from a box.
            while !(unsafe { (*ptr).next }.is_null()) && i < index {
                i += 1;
                //SAFETY: Dereferencing this pointer should be okay since
                // it was obtained from a box.
                ptr = unsafe { (*ptr).next };
            }
            if i == index {
                //SAFETY: Dereferencing these pointers should be okay since
                // they were allocated by boxes.
                unsafe {
                    (*(*ptr).prev).next = new_ptr;
                    (*new_ptr).prev = (*ptr).prev;
                    (*ptr).prev = new_ptr;
                    (*new_ptr).next = ptr;
                };
            } else {
                //Index is outside of the list, so append value
                //to the list
                //SAFETY: Dereferencing these pointers should be okay since
                // they were allocated by boxes.
                unsafe {
                    (*ptr).next = new_ptr;
                    (*new_ptr).prev = ptr;
                };
            }
        }
    }

    pub fn append(&mut self, value: T) {
        self.insert(value, self.len());
    }
}

#[cfg(test)]
mod tests {
    use super::DoublyLinkedList;
    #[test]
    fn test_append() {
        let mut list = DoublyLinkedList::new();
        list.append(4);
        assert_eq!(*list.get(0).unwrap(), 4);
    }

    #[test]
    fn test_size() {
        let mut list = DoublyLinkedList::new();
        assert_eq!(list.len(), 0);
        list.append(4);
        assert_eq!(list.len(), 1);
        list.append(4);
        assert_eq!(list.len(), 2);
        list.append(4);
        assert_eq!(list.len(), 3);
    }

    #[test]
    fn test_get() {
        let mut list = DoublyLinkedList::new();
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

    #[test]
    fn test_insert_at_head() {
        let mut list = DoublyLinkedList::new();
        list.append(2);
        list.append(3);
        list.append(4);
        list.append(5);
        list.insert(1, 0);
        assert_eq!(list.len(), 5);
        assert_eq!(*list.get(0).unwrap(), 1);
        assert_eq!(*list.get(1).unwrap(), 2);
    }

    #[test]
    fn test_insert_at_between() {
        let mut list = DoublyLinkedList::new();
        list.append(2);
        list.append(3);
        list.append(4);
        list.append(5);
        list.insert(10, 2);
        assert_eq!(list.len(), 5);
        assert_eq!(*list.get(1).unwrap(), 3);
        assert_eq!(*list.get(2).unwrap(), 10);
        assert_eq!(*list.get(3).unwrap(), 4);
    }

    #[test]
    fn test_insert_at_tail() {
        let mut list = DoublyLinkedList::new();
        list.append(2);
        list.append(3);
        list.append(4);
        list.append(5);
        list.insert(8, 3);
        assert_eq!(list.len(), 5);
        assert_eq!(*list.get(3).unwrap(), 8);
        assert_eq!(*list.get(4).unwrap(), 5);
    }

    /*
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

    */
}
