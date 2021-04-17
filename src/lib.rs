pub mod ds {
    mod doubly_linked_list;
    mod linked_list;
    mod stack;
    pub use doubly_linked_list::DoublyLinkedList;
    pub use linked_list::LinkedList;
    pub use stack::Stack;
}
