struct Node<T> {
    value: T,
    level: usize,
    children: Vec<Self>,
}

impl<T> Node<T> {
    fn new(value: T, level: usize) -> Self {
        Self {
            value,
            level,
            children: Vec::default(),
        }
    }
}

fn build<T>(root: T, items: Vec<(T, usize)>) -> Node<T> {
    let mut stack = vec![Node::new(root, 0)];
    for (value, level) in items {
        while let Some(child) = stack.pop_if(|last| last.level >= level) {
            stack.last_mut().unwrap().children.push(child);
        }
        stack.push(Node::new(value, level));
    }
    while stack.len() > 1 {
        let child = stack.pop().unwrap();
        stack.last_mut().unwrap().children.push(child);
    }
    stack.pop().unwrap()
}

fn display<T: std::fmt::Display>(node: &Node<T>, level: usize) {
    let indent = level * 2;
    println!("{:indent$}{}", "", node.value);
    for child in &node.children {
        display(child, level + 1);
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
    let root = build("ROOT", items);
    display(&root, 0);
}
