// A doubly linked list or a two-way linked list is a more complex type of linked list which contains
// a pointer to the next as well as the previous node in the sequence. Therefore, it consists of three
// parts—data, a pointer to the next node, and a pointer to the previous node

// Since each node will be pointed at by two other nodes,
// we will use raw pointers to avoid dealing with the hassle
// of having to use the interior mutability pattern

struct Node<T> {
    prev: *mut Node<T>,
    value: T,
    next: *mut Node<T>,
}

pub struct DoublyLinkedList<T> {
    head: *mut Node<T>,
    len: usize,
}

impl<T> Drop for DoublyLinkedList<T> {
    fn drop(&mut self) {
        let mut ptr = self.head;
        //SAFETY: Dereferencing this pointer should be okay since
        // it was obtained from a box.
        while !ptr.is_null() {
            //SAFETY: Getting a box from this pointer is okay
            //because the pointer was obtained from another box.
            let node = unsafe { Box::from_raw(ptr) };
            ptr = node.next;
        }
    }
}

impl<T> DoublyLinkedList<T> {
    pub fn new() -> DoublyLinkedList<T> {
        DoublyLinkedList {
            head: std::ptr::null_mut(),
            len: 0,
        }
    }

    //Returns the number of elements in the list
    pub fn len(&self) -> usize {
        self.len
    }

    //Returns a reference to the element at the specified position.
    //Panics if index is large than list.len()
    pub fn get(&self, index: usize) -> &T {
        if index >= self.len {
            panic!("Index is larger than the length of the list")
        }
        let mut i = 0;
        let mut ptr = self.head;
        while !ptr.is_null() {
            if i == index {
                break;
            }
            //SAFETY: Deref is okay, pointer was obtained from box
            ptr = unsafe { (*ptr).next };
            i += 1;
        }
        //SAFETY: Deref is okay, pointer was obtained from box
        // Pointer should not be null as the length of the list
        // guarantees that only valid pointers are accessed
        unsafe { &(*ptr).value }
    }

    //Inserts a value at the given position.
    //If index is larger than list.len(), value is appended
    //to the end of the list.
    pub fn insert(&mut self, value: T, index: usize) {
        let new_node = Box::new(Node {
            prev: std::ptr::null_mut(),
            value,
            next: std::ptr::null_mut(),
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
        self.len += 1;
    }

    pub fn append(&mut self, value: T) {
        self.insert(value, self.len);
    }

    //Removes the element at the given index and returns it
    pub fn remove(&mut self, index: usize) -> T {
        //SAFETY: Deref is okay, pointer was obtained from box
        // Pointer should not be null as the length of the list
        // guarantees that only valid pointers are accessed
        if index >= self.len {
            panic!("Index is larger than the length of the list");
        }
        if index == 0 {
            //SAFETY: Getting a box from this pointer is okay
            //because the pointer was obtained from another box.
            let node = unsafe { Box::from_raw(self.head) };
            self.head = node.next;
            self.len -= 1;
            node.value
        } else {
            let mut ptr = self.head;
            let mut i = 0;
            while i < index {
                i += 1;
                //SAFETY: Dereferencing this pointer should be okay since
                // it was obtained from a box and this section of the code.
                // will only be run for non null pointers
                ptr = unsafe { (*ptr).next };
            }
            //SAFETY: Getting a box from this pointer is okay
            //because the raw pointer was obtained from another box.
            let node = unsafe { Box::from_raw(ptr) };
            if i == self.len - 1 {
                //SAFETY: Dereferencing this pointers should be okay since
                // it was obtained from a box and should be non null.
                unsafe {
                    (*node.prev).next = std::ptr::null_mut();
                };
            } else {
                //SAFETY: Dereferencing these pointers should be okay since
                // there were obtained from boxes and should non null.
                unsafe {
                    (*node.prev).next = node.next;
                    (*node.next).prev = node.prev;
                };
            }
            self.len -= 1;
            node.value
        }
    }
}

#[cfg(test)]
mod tests {
    use super::DoublyLinkedList;
    #[test]
    fn test_append() {
        let mut list = DoublyLinkedList::new();
        list.append(4);
        assert_eq!(*list.get(0), 4);
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
        list.append(4);
        assert_eq!(*list.get(0), 4);
        list.append(5);
        assert_eq!(*list.get(1), 5);
        list.append(6);
        assert_eq!(*list.get(2), 6);
    }

    #[test]
    #[should_panic(expected = "Index is larger than the length of the list")]
    fn test_get_panic() {
        let list = DoublyLinkedList::<i32>::new();
        list.get(0);
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
        assert_eq!(*list.get(0), 1);
        assert_eq!(*list.get(1), 2);
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
        assert_eq!(*list.get(1), 3);
        assert_eq!(*list.get(2), 10);
        assert_eq!(*list.get(3), 4);
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
        assert_eq!(*list.get(3), 8);
        assert_eq!(*list.get(4), 5);
    }

    #[test]
    #[should_panic(expected = "Index is larger than the length of the list")]
    fn test_remove_panic() {
        let mut list = DoublyLinkedList::<i32>::new();
        list.remove(1);
    }

    #[test]
    fn test_remove_at_head() {
        let mut list = DoublyLinkedList::new();
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
        let mut list = DoublyLinkedList::new();
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
        let mut list = DoublyLinkedList::new();
        list.append(1);
        list.append(4);
        list.append(5);
        assert_eq!(list.len(), 3);
        assert_eq!(list.remove(2), 5);
        assert_eq!(*list.get(1), 4);
    }
}
