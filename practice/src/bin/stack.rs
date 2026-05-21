use std::io::{self};

struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Stack { items: Vec::new() }
    }

    fn push(&mut self, value: T) {
        self.items.push(value);
    }

    fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }
    #[allow(dead_code)]
    fn size(&self) -> usize {
        self.items.len()
    }

    fn top(&self) -> Option<&T> {
        self.items.last()
    }
}

fn main() {
    let mut st: Stack<i32> = Stack::new();

    let mut eqn = String::new();

    io::stdin()
        .read_line(&mut eqn)
        .expect("Failed to read input");

    for token in eqn.split_whitespace() {
        if let Ok(num) = token.parse::<i32>() {
            st.push(num);
        } else {
            let b = st.pop().unwrap();
            let a = st.pop().unwrap();

            let result = match token {
                "+" => a + b,
                "-" => a - b,
                "*" => a * b,
                "/" => a / b,
                _ => panic!("Invalid operator"),
            };

            st.push(result);
        }
    }
    println!("{}", st.top().unwrap());
}
