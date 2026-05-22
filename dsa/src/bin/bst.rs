struct Node<T> {
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

struct Bst<T> {
    root: Option<Box<Node<T>>>,
}

impl<T: Ord + std::fmt::Display> Bst<T> {
    fn new() -> Self {
        Self { root: None }
    }
    fn insert(&mut self, value: T) {
        Self::insert_node(&mut self.root, value);
    }
    fn insert_node(node: &mut Option<Box<Node<T>>>, value: T) {
        match node {
            Some(current) => {
                if value < current.value {
                    Self::insert_node(&mut current.left, value);
                } else {
                    Self::insert_node(&mut current.right, value);
                }
            }
            None => {
                *node = Some(Box::new(Node {
                    value,
                    left: None,
                    right: None,
                }));
            }
        }
    }
    fn contains(&self, value: &T) -> bool {
        Self::search(&self.root, value)
    }
    fn search(node: &Option<Box<Node<T>>>, value: &T) -> bool {
        match node {
            Some(current) => {
                if *value == current.value {
                    true
                } else if *value > current.value {
                    Self::search(&current.right, value)
                } else {
                    Self::search(&current.left, value)
                }
            }
            None => false,
        }
    }
    fn inorder(&self) {
        Self::inorder_traversal(&self.root);
        println!();
    }

    fn inorder_traversal(node: &Option<Box<Node<T>>>) {
        if let Some(current) = node {
            Self::inorder_traversal(&current.left);
            print!("{} ", current.value);
            Self::inorder_traversal(&current.right);
        }
    }
}

fn main() {
    let mut root: Bst<i32> = Bst::new();
    root.insert(50);
    root.insert(30);
    root.insert(70);
    root.insert(20);
    root.insert(40);

    root.inorder();

    println!("Contains 40: {}", root.contains(&40));
    println!("Contains 100: {}", root.contains(&100));
}
