mod tree;
use tree::TreeNode;
use std::cell::RefCell;
use std::sync::Arc;

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn read_file() -> String {
    println!("Input file name:");
    let mut file_name = String::new();
    std::io::stdin().read_line(&mut file_name).expect("Failed to read input");

    file_name = file_name.trim().to_string();

    print!("Reading {}", file_name);

    let mut file_contents = std::fs::read_to_string(file_name)
        .expect("Should have been able to read the file");

    print_type_of(&file_contents);
    file_contents
}

#[derive(Debug, Clone)]
struct Element {
    is_int : bool,
    val : i32,
}

fn parse_input(contents: &str) {
    let mut vec : Vec<Arc<RefCell<TreeNode<Element>>>> = Vec::<Arc<RefCell<TreeNode<Element>>>>::new();
    for line in contents.lines() {
        let mut dummy = TreeNode::get_new_tree(Element{is_int:false, val:0});
        let mut curr_node = dummy.clone();
        if line.trim() == "" {println!(""); continue;}
        line.split(',')
            .for_each(|sub|
                {
                    let removed_open = sub.trim().trim_start_matches('[');
                    let removed_close = removed_open.trim_end_matches(']').trim();
                    let num_open = sub.len() - removed_open.len();
                    let num_close = sub.len() - removed_close.len() - num_open;
                    for _ in 0..num_open {
                        let new_node = TreeNode::get_new_tree(Element{is_int:false, val:0});
                        TreeNode::add_child_to_self(curr_node.clone(), new_node.clone());                                 
                        curr_node = new_node;
                    }
                    if let Ok(n) = removed_close.parse::<i32>() {
                        TreeNode::add_child_to_self(curr_node.clone(), TreeNode::get_new_tree(Element{is_int:true, val:n}));
                    }
                    for _ in 0..num_close {
                        curr_node = curr_node.clone().borrow_mut().parent.as_ref().unwrap().clone();
                    }
                }

            );
        println!("{line}");
        dummy.borrow().print_whole_tree();
    }
}

fn main() {
    let contents = read_file();

    parse_input(&contents);
}