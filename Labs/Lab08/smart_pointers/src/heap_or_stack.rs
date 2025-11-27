use std::ops::{Deref, DerefMut};

pub enum HeapOrStack<T> {
    Stack(T),
    Heap(Box<T>),
}

impl<T> Deref for HeapOrStack<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match self {
            HeapOrStack::Stack(value) => value,
            HeapOrStack::Heap(boxed_value) => boxed_value,
        }
    }
}

impl<T> DerefMut for HeapOrStack<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            HeapOrStack::Stack(value) => value,
            HeapOrStack::Heap(boxed_value) => boxed_value,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::HeapOrStack;

    #[test]
    fn test_heap_or_stack() {
        let mut stack_value = HeapOrStack::Stack(10);
        let mut heap_value = HeapOrStack::Heap(Box::new(20));

        assert_eq!(*stack_value, 10);
        assert_eq!(*heap_value, 20);

        *stack_value += 5;
        *heap_value += 5;

        assert_eq!(*stack_value, 15);
        assert_eq!(*heap_value, 25);
    }
}
