struct Node<T> {
    value: T,
    children: Vec<Self>,
}

impl<T> Node<T> {
    fn new(value: T) -> Self {
        Self {
            value,
            children: Vec::default(),
        }
    }
}

fn build<T>(root: T, items: Vec<(T, usize)>) -> Node<T> {
    fn append_child<U>(stack: &mut Vec<(Node<U>, usize)>) {
        let (child, _) = stack.pop().unwrap();
        stack.last_mut().unwrap().0.children.push(child);
    }

    let mut stack = vec![(Node::new(root), 0)];
    for (value, level) in items {
        while stack.last().is_some_and(|(_, l)| *l >= level) {
            append_child(&mut stack);
        }
        stack.push((Node::new(value), level));
    }
    while stack.len() > 1 {
        append_child(&mut stack);
    }
    stack.pop().unwrap().0
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
