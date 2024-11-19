#[derive(Debug, Clone)]
pub struct Stack<T> {
    top: Option<Box<StackNode<T>>>,
    len: u32,
}

#[derive(Debug, Clone)]
pub struct StackNode<T> {
    val: T,
    next: Option<Box<StackNode<T>>>,
}

impl<T> Stack<T> {
    fn new() -> Stack<T> {
        Stack { top: None, len: 0 }
    }

    fn pop(&mut self) -> Option<T> {
        let val = self.top.take();
        match val {
            Some(mut v) => {
                self.top = v.next.take();
                self.len -= 1;
                Some(v.val)
            }
            None => None,
        }
    }

    fn push(&mut self, val: T) {
        let mut new_node = StackNode::new(val);
        new_node.next = self.top.take();
        self.top = Some(Box::new(new_node));
        self.len += 1;
    }

    fn is_empty(&self) -> bool {
        self.len == 0
    }
}

impl<T> StackNode<T> {
    fn new(v: T) -> StackNode<T> {
        StackNode { val: v, next: None }
    }
}

#[test]
fn test_stack() {
    #[derive(PartialEq, Eq, Debug)]
    struct TestVal {
        v: i32,
    }

    let a = TestVal { v: 1 };
    let b = TestVal { v: 2 };
    let c = TestVal { v: 3 };

    let mut stack = Stack::<TestVal>::new();
    assert_eq!(true, stack.is_empty());

    stack.push(a);
    stack.push(b);
    stack.push(c);

    assert_eq!(3, stack.len);

    assert_eq!(Some(TestVal { v: 3 }), stack.pop());
    println!("{:?}", stack.pop());
    println!("{:?}", stack.pop());
}
