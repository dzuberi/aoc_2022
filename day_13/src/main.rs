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
                    println!(",: {sub}");
                    if let Ok(i) = sub.parse::<i32>() {
                        TreeNode::add_child_to_self(curr_node.clone(), TreeNode::get_new_tree(Element{is_int: true, val: i}));
                    }
                    else {
                        dummy.borrow().print_whole_tree();
                        // if sub.contains('['){
                        sub.split('[')
                            .filter(|s| s.trim() != "")
                            .for_each(|sub1|
                                {
                                    println!("[: {sub1}");
                                    if let Ok(i) = sub1.parse::<i32>() {
                                        TreeNode::add_child_to_self(curr_node.clone(), TreeNode::get_new_tree(Element{is_int:true, val:i}));
                                    }
                                    else {
                                        curr_node = TreeNode::get_new_tree(Element{is_int:false, val:0});
                                        TreeNode::add_child_to_self(dummy.clone(), curr_node.clone());                                    
                                        dummy.borrow().print_whole_tree();
                                        //if sub1.contains(']'){
                                        sub1.split(']')
                                            .filter(|s| s.trim() != "")
                                            .for_each(|sub2|
                                                {
                                                    println!("]: {sub2}");
                                                    if let Ok(i) = sub2.parse::<i32>() {
                                                        TreeNode::add_child_to_self(curr_node.clone(), TreeNode::get_new_tree(Element{is_int:true, val:i}));
                                                    }
                                                    else {
                                                        curr_node = curr_node.clone().borrow_mut().parent.as_ref().unwrap().clone();
                                                    }
                                                dummy.borrow().print_whole_tree();
                                                }
                                            )//}
                                    }
                                }
                            )//}
                        }
                    dummy.borrow().print_whole_tree();
                }

            );
        println!("{line}");
        dummy.borrow().print_whole_tree();
    }
}

fn main() {
    let contents = read_file();

    parse_input(&contents);

    let tree = TreeNode::get_new_tree(0i32);

    TreeNode::add_child_to_self(tree.clone(), TreeNode::get_new_tree(1i32));
    
    println!("");
    
    tree.borrow().print_self();
    
    println!("");

    tree.borrow().children[0].borrow().print_whole_tree();
}