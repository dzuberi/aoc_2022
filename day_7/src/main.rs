use std::vec::Vec;
use std::ptr;
use std::rc::Rc;
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

#[derive(Debug)]
struct TreeNode{
    parent: Option<Arc<RefCell<TreeNode>>>,
    children: Vec<Arc<RefCell<TreeNode>>>,
    size: i32,
    name: String,
    t: String,
}

impl TreeNode{
    fn add_child(&mut self, new_child : Arc<RefCell<TreeNode>>){
        //let mut child_node = new_child.borrow_mut();

        self.children.push(
            new_child
        );
    }

    fn has_children(&mut self)->bool {
        return !self.children.is_empty();
    }
}

fn build_tree_from_contents(contents : &str) -> Arc<RefCell<TreeNode>> {
    let dummy = Arc::new( RefCell::new(TreeNode {parent: None, children: Vec::<Arc<RefCell<TreeNode>>>::new(), size: 0, name:"dummy".to_string(), t:"dummy".to_string()} ) );
    
    
    let mut current_level = -1i32;
    let mut parent = dummy.clone();
    for line in contents.lines(){
        println!("{line}");
        let level = line.find('-').unwrap() as i32;
        println!("{level}, {current_level}");
        let name = line.split('-').nth(1).unwrap().split(' ').nth(1).unwrap();
        let info = line.split('(').nth(1).unwrap().split(')').nth(0).unwrap();
        let t = info.split(',').nth(0).unwrap().trim();
        let mut sz = 0;
        match info.split(',').nth(1) {
            Some(s) => {
                sz = s.split('=').nth(1).unwrap().trim().parse::<i32>().unwrap();
            }
            None => {
                sz = 0;
            }
        }
        /*
        if level == current_level {
            parent = parent.clone().borrow().parent.as_ref().unwrap().clone();

        }
        */
        while level <= current_level {
            parent = parent.clone().borrow().parent.as_ref().unwrap().clone();
            current_level -= 1;

        }
        let new_child = Arc::new( RefCell::new(TreeNode {parent: Some(parent.clone()), children: Vec::<Arc<RefCell<TreeNode>>>::new(), size: sz, name:name.to_string(), t:t.to_string()} ) );
        parent.borrow_mut().add_child(new_child.clone());
        current_level = level;
        parent = new_child.clone();
        //print_tree(&dummy, 0);
    }
    
    return dummy;
}

fn print_tree(head: &Arc<RefCell<TreeNode>>, level: u32) {
    let name = &head.borrow().name;
    let size = head.borrow().size;
    let t = &head.borrow().t;
    for i in 0..level{
        print!(" ");
    }
    println!("{name}, {t}, {size}");
    for child in head.borrow().children.iter(){
        print_tree(child, level+1);
    }
}

fn pop_size(head: &Arc<RefCell<TreeNode>>) -> i32 {
    let mut size = 0;
    {
        let name = &head.borrow().name;
        size = head.borrow().size.clone();
        for child in head.borrow().children.iter(){
            size += pop_size(child);
        }
    }
    head.clone().borrow_mut().size = size;
    return size;
}

fn do_part_1(head: &Arc<RefCell<TreeNode>>) -> i32 {
    let mut total = 0;
    {    
        let t = &head.borrow().t;
        let name = &head.borrow().name;
        let size = head.borrow().size.clone();
        for child in head.borrow().children.iter(){
            total += do_part_1(child);
        }
        if t == "dir" && size <= 100000  {total += size};
    }
    return total;
}


fn do_part_2(head: &Arc<RefCell<TreeNode>>, result: i32, min_size: i32) -> i32 {
    let mut retval = result;
    {    
        let t = &head.borrow().t;
        let name = &head.borrow().name;
        let size = head.borrow().size.clone();
        if t.contains("dir") {println!("result: {result}, min_size: {min_size}, size:{size}");}
        for child in head.borrow().children.iter(){
            retval = do_part_2(child, retval, min_size);
        }
        if t.contains( "dir") && size >= min_size && size < retval  {retval = size};
    }
    return retval;
}


fn process_file_tree_string(contents: &str) {

    let mut tree = build_tree_from_contents(&contents);
    print_tree(&tree, 0);
    pop_size(&tree);
    print_tree(&tree, 0);
    let part_1 = do_part_1(&tree);
    let part_2 = do_part_2(&tree, tree.borrow().size, 30000000-(70000000-tree.borrow().size));
    println!("part_1: {part_1}\npart_2: {part_2}");

}

fn convert_input(contents: &str) -> String {
    let mut string = String::new();
    let mut level = -1;
    for line in contents.lines(){
        let mut split = line.split(' ');
        let mut cmd = "";
        let first = split.next().unwrap();
        let mut second = "";
        match split.next(){
            Some(s) => second=s,
            None=>{},
        }
        let mut third = "";
        match split.next(){
            Some(s) => third=s,
            None=>{},
        }
        if first.starts_with("$"){
            cmd = second;
        }
        if cmd == "cd" {
            if third.contains(".."){
                level -= 1;
            }
            else {
                level += 1;
                for i in 0..level{
                    string.push_str(" ");
                }
                string.push_str("- ");
                string.push_str(third);
                string.push_str(" (dir)\n");
            }
            //continue;
        }
        match first.parse::<i32>(){
            Ok(s) => {
                let level_inc = level+1;
                for i in 0..level_inc{
                    string.push_str(" ");
                }
                string.push_str("- ");
                string.push_str(second);
                string.push_str(" (file, size=");
                string.push_str(first);
                string.push_str(")\n");
            }
            Err(..) => {}
        }
        println!("first: {first}, second: {second}, third: {third}, level: {level}");
    }
    return string;
}

fn main() {
    let mut contents = read_file();
    let tree_string = convert_input(&contents);
    process_file_tree_string(&tree_string);
}