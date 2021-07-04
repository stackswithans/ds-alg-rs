pub mod ds {
    mod bst;
    mod doubly_linked_list;
    mod linked_list;
    mod queue;
    mod stack;
    pub use bst::BST;
    pub use doubly_linked_list::DoublyLinkedList;
    pub use linked_list::LinkedList;
    pub use queue::Queue;
    pub use stack::Stack;
}
