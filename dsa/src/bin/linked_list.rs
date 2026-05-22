struct Node<T> {
    val: T,
    next: Option<Box<Node<T>>>,
}

struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> LinkedList<T> {
    fn new() -> Self {
        Self { head: None }
    }

    fn push_front(&mut self, val: T) {
        let new_node = Box::new(Node {
            val,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }

    fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.val
        })
    }
    fn print(&self)
    where
        T: std::fmt::Display,
    {
        let mut current = &self.head;

        while let Some(node) = current {
            print!("{} -> ", node.val);
            current = &node.next;
        }

        println!("None");
    }
}
fn main() {
    let mut list = LinkedList::new();

    list.push_front("abxc");
    list.push_front("abxc");
    list.push_front("abxc");

    list.print();

    if let Some(val) = list.pop_front() {
        println!("Popped: {}", val);
    }

    list.print();
}
