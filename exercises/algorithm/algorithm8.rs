/*
	queue
	This question requires you to use queues to implement the functionality of the stack
*/


#[derive(Debug)]
pub struct Queue<T> {
    elements: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }

    pub fn enqueue(&mut self, value: T) {
        self.elements.push(value)
    }

    pub fn dequeue(&mut self) -> Result<T, &str> {
        if !self.elements.is_empty() {
            Ok(self.elements.remove(0usize))
        } else {
            Err("Queue is empty")
        }
    }

    pub fn peek(&self) -> Result<&T, &str> {
        match self.elements.first() {
            Some(value) => Ok(value),
            None => Err("Queue is empty"),
        }
    }

    pub fn size(&self) -> usize {
        self.elements.len()
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}

impl<T> Default for Queue<T> {
    fn default() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }
}

pub struct myStack<T>
{
    // TODO: Implement myStack using two queues (q1 and q2)
    q1: Queue<T>,
    q2: Queue<T>,
}

impl<T> myStack<T> {
    pub fn new() -> Self {
        Self {
            q1: Queue::<T>::new(),
            q2: Queue::<T>::new(),
        }
    }

    pub fn push(&mut self, elem: T) {
        // TODO: Implement the push operation using queues
        self.q1.enqueue(elem);
    }

    pub fn pop(&mut self) -> Result<T, &str> {
        // TODO: Implement the pop operation using queues
        if self.q1.is_empty() {
            return Err("Stack is empty");
        }

        // Move elements from q1 to q2 except the last one
        while self.q1.size() > 1 {
            if let Ok(front) = self.q1.dequeue() {
                self.q2.enqueue(front);
            }
        }

        // Retrieve the last element in q1 (which is the top of the stack)
        if let Ok(top) = self.q1.dequeue() {
            // Swap q1 and q2 to restore the original state
            std::mem::swap(&mut self.q1, &mut self.q2);
            return Ok(top);
        }

        Err("Stack is empty")
    }

    pub fn is_empty(&self) -> bool {
        // TODO: Implement the is_empty operation using queues
        self.q1.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stack() {
        let mut stack = myStack::<i32>::new();
        assert_eq!(stack.pop(), Err("Stack is empty"));
        stack.push(1);
        stack.push(2);
        stack.push(3);
        assert_eq!(stack.pop(), Ok(3));
        assert_eq!(stack.pop(), Ok(2));
        stack.push(4);
        stack.push(5);
        assert_eq!(stack.is_empty(), false);
        assert_eq!(stack.pop(), Ok(5));
        assert_eq!(stack.pop(), Ok(4));
        assert_eq!(stack.pop(), Ok(1));
        assert_eq!(stack.pop(), Err("Stack is empty"));
        assert_eq!(stack.is_empty(), true);
    }
}
