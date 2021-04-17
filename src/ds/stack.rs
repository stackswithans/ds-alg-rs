//A stack is a linear data structure in which elements
//are added and removed only from one end, which
//is called the top. Hence, a stack is called a LIFO
//(Last-In, First-Out) data structure as the element
//that is inserted last is the first one to be taken out.

use super::linked_list::LinkedList;

pub struct Stack<T> {
    items: LinkedList<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack {
            items: LinkedList::new(),
        }
    }

    //Pushes a value onto the stack
    pub fn push(&mut self, value: T) {
        self.items.insert(value, 0);
    }

    //Removes the topmost element from the stack and returns it.
    //Panics if the stack is empty.
    pub fn pop(&mut self) -> T {
        if self.items.len() == 0 {
            panic!("Invalid pop! The stack is empty");
        }
        self.items.remove(0)
    }

    //Returns a reference to the topmost element of the stack
    //Panics if the stack is empty.
    pub fn peek(&self) -> &T {
        if self.items.len() == 0 {
            panic!("Invalid peek! The stack is empty");
        }
        self.items.get(0)
    }
}

#[cfg(test)]
mod test {
    use super::Stack;

    #[test]
    fn test_push() {
        let mut stack = Stack::new();
        stack.push(1);
        assert_eq!(*stack.items.get(0), 1);
        stack.push(2);
        assert_eq!(*stack.items.get(0), 2);
        stack.push(3);
        assert_eq!(*stack.items.get(0), 3);
    }

    #[test]
    fn test_pop() {
        let mut stack = Stack::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);
        assert_eq!(stack.pop(), 3);
        assert_eq!(stack.pop(), 2);
        assert_eq!(stack.pop(), 1);
    }

    #[test]
    #[should_panic(expected = "Invalid pop! The stack is empty")]
    fn test_pop_panic() {
        let mut stack = Stack::<i32>::new();
        stack.pop();
    }

    #[test]
    fn test_peek() {
        let mut stack = Stack::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);
        assert_eq!(*stack.peek(), 3);
        stack.pop();
        assert_eq!(*stack.peek(), 2);
        stack.pop();
        assert_eq!(*stack.peek(), 1);
    }

    #[test]
    #[should_panic(expected = "Invalid peek! The stack is empty")]
    fn test_peek_panic() {
        let stack = Stack::<i32>::new();
        stack.peek();
    }
}
