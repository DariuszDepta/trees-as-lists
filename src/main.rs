use std::fmt::Display;

struct Node<T>
where
    T: Display,
{
    value: T,
    level: usize,
    children: Vec<Self>,
}

impl<T: Display> Node<T> {
    fn new(value: T, level: usize) -> Self {
        Self {
            value,
            level,
            children: Vec::default(),
        }
    }
}

impl<T: Display> Display for Node<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let indent = self.level * 2;
        writeln!(f, "{:indent$}{}", "", self.value)?;
        for child in &self.children {
            write!(f, "{child}")?;
        }
        Ok(())
    }
}

fn main() {
    let items = vec![
        ("A", 1),
        ("B", 2),
        ("C", 2),
        ("D", 1),
        ("E", 2),
        ("F", 3),
        ("G", 1),
        ("H", 1),
        ("I", 2),
        ("J", 3),
        ("K", 4),
    ];

    let mut stack = vec![Node::new("ROOT", 0)];

    for (value, level) in items {
        while stack.last().is_some_and(|last| last.level >= level) {
            let child = stack.pop().unwrap();
            stack.last_mut().unwrap().children.push(child);
        }
        stack.push(Node::new(value, level));
    }
    while stack.len() > 1 {
        let child = stack.pop().unwrap();
        stack.last_mut().unwrap().children.push(child);
    }

    let root = stack.pop().unwrap();
    println!("{root}");
}
