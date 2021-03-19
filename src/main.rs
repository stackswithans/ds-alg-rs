/* A node in the linked list, This struct is generic over the type of
 * the linked list
 */ 
use core::fmt::Display;

struct Node<T>{
    value : T,
    next : Option<Box<Node<T>>>
}

struct LinkedList<T>{
    head : Option<Box<Node<T>>>
}

impl<T> LinkedList<T>{
    fn new() -> LinkedList<T>{
        LinkedList{
            head: None, 
        }
    }

    // Method that inserts a value into the linked list
    fn insert(&mut self, value: T){
        let new_node = Node{
            value,
            next: None
        };
        match self.head.as_mut() {
            Some(head)=>{
                let mut node : &mut Box<Node<T>> = head;
                while node.next.is_some(){
                     node = node.next.as_mut().unwrap();
                }
                node.next = Some(Box::new(new_node)); 
            }
            None => {
                self.head = Some(Box::new(new_node));
            }
        }
    }

    /*Method that removes a value from the list
     * Returns true if value was in the list and
     * succesfully removed. Returns false in case value was
     * not found.
     */
    //Do i need ownership to delete a value?
    fn remove(&mut self, value : T) -> bool
    where
    T: PartialOrd{
        let mut node : &mut Option<Box<Node<T>>> = &mut self.head;
        let retrivied_node :  Box<Node<T>> = node.unwrap();

        false
    }

    fn print(&self)
    where
    T: Display{
        let mut node : &Option<Box<Node<T>>> = &self.head; 
        while node.is_some(){
            let value : &T = &node.as_ref().unwrap().value;
            println!("{}", value);
            node = &node.as_ref().unwrap().next;
        }
    }
}

fn main(){
    let mut list = LinkedList::new();
    list.insert(1);
    list.insert(2);
    list.insert(3);
    list.insert(4);
    list.print();
}
