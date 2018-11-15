#[derive(Debug,Clone)]
pub struct Stack<T> {
    top: Option<Box<StackNode<T>>>,
    len:u64,
}

#[derive(Debug,Clone)]
pub struct StackNode<T> {
    val: T,
    next: Option<Box<StackNode<T>>>,
}

impl<T> StackNode<T> {
    pub fn new(val:T) -> StackNode<T> {
        StackNode{ val:val, next:None}
    }
}

impl<T> Stack<T> {
    pub fn new() -> Stack<T>{
        Stack {top:None, len:0}
    }

    pub fn push(&mut self, val:T) {
        let mut new_node = StackNode::new(val);
        new_node.next = self.top.take();
        self.top = Some(Box::new(new_node));
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        let val = self.top.take();
        match val {
            Some(mut v) => {
                self.top = v.next.take();
                self.len -= 1;
                Some(v.val)
            },
            None => None,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

#[test]
fn test_stack() {
    #[derive(PartialEq,Eq,Debug)]
    struct TestVal{
        v:i32,
    }

    let a = TestVal{v:1};
    let b = TestVal{v:2};
    let c = TestVal{v:3};

    let mut stack = Stack::<TestVal>::new();
    assert_eq!(true, stack.is_empty());

    stack.push(a);
    stack.push(b);
    stack.push(c);

    assert_eq!(3, stack.len);

    assert_eq!(Some(TestVal{v:3}),stack.pop());
    println!("{:?}",stack.pop());
    println!("{:?}",stack.pop());
}