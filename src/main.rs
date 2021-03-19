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

    fn add_node(&mut self, value: T){
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
                let node : &mut Node<T> = &mut node.as_mut();
                node.next = Some(Box::new(new_node)); 
            }
            None => {
                self.head = Some(Box::new(new_node));
            }
        }
    }

    fn delete_node(&self)

    fn print_list(&self)
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
    list.add_node(1);
    list.add_node(2);
    list.add_node(3);
    list.add_node(4);
    list.print_list();
}
