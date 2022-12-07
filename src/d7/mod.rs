use std::{cell::RefCell, collections::HashMap, rc::Rc};

pub mod p1;
pub mod p2;

const TOTAL_SIZE: usize = 70_000_000;

const SIZE_NEEDED: usize = 30_000_000;

const SMALL_FILE_LIMIT: usize = 100_000;

const RAW_DATA: &'static str = include_str!("./data");

struct Node {
    pub size: Option<usize>,
    pub is_file: bool,
    pub children: HashMap<String, Rc<RefCell<Node>>>,
    pub parent: Option<Rc<RefCell<Node>>>,
}

impl Node {
    pub fn new() -> Node {
        Node {
            size: None,
            is_file: false,
            children: HashMap::new(),
            parent: None,
        }
    }
}

fn data() -> Rc<RefCell<Node>> {
    parse_input(RAW_DATA)
}

fn parse_input(lines: &str) -> Rc<RefCell<Node>> {
    let root = Rc::new(RefCell::new(Node::new()));
    let mut cur_node = Rc::clone(&root);
    for line in lines.lines() {
        let tokenz: Vec<&str> = line.split(' ').collect();
        if line.starts_with('$') {
            cur_node = parse_command(line, &tokenz, &cur_node, &root);
        } else if let [size_or_dir, name] = &tokenz[..] {
            parse_ls_output(size_or_dir, &cur_node, name);
        }
    }
    root
}

fn parse_ls_output(size_or_dir: &&str, cur_node: &Rc<RefCell<Node>>, name: &&str) {
    if !cur_node.borrow().children.contains_key(*name) {
        insert_child(size_or_dir, cur_node, name);
    }
}

fn insert_child(size_or_dir: &&str, cur_node: &Rc<RefCell<Node>>, name: &&str) {
    let child = Rc::new(RefCell::new(Node::new()));
    let mut mut_child = child.borrow_mut();
    if *size_or_dir != "dir" {
        mut_child.is_file = true;
        mut_child.size = Some(size_or_dir.parse().unwrap());
    }
    mut_child.parent = Some(Rc::clone(cur_node));
    cur_node
        .borrow_mut()
        .children
        .insert(name.to_string(), Rc::clone(&child));
}

fn parse_command(
    line: &str,
    tokenz: &[&str],
    cur_node: &Rc<RefCell<Node>>,
    root: &Rc<RefCell<Node>>,
) -> Rc<RefCell<Node>> {
    if !line.contains("cd") {
        return cur_node.clone();
    }
    let folder = tokenz[2];
    match folder {
        ".." => Rc::clone(cur_node.borrow().parent.as_ref().unwrap()),
        "/" => root.clone(),
        _ => cur_node.borrow().children.get(folder).unwrap().clone(),
    }
}

fn calc_sum<'a>(node: &'a Node, sizes: &'a mut Vec<usize>) -> (usize, &'a mut Vec<usize>) {
    if node.is_file {
        return (node.size.unwrap(), sizes);
    }
    let sum_c = node
        .children
        .values()
        .map(|child| calc_sum(&child.borrow(), sizes).0)
        .sum();
    sizes.push(sum_c);
    (sum_c, sizes)
}
