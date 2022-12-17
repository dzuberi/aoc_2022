use std::vec::Vec;
use std::ptr;

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
struct TreeNode<'a>{
    parent: *mut TreeNode<'a>,
    children: Vec<*mut TreeNode<'a>>,
    size: i32,
    name: String,
}

impl<'a> TreeNode<'a> {
    fn add_child<'b>(&mut self, new_node : *mut TreeNode<'a>){
        unsafe {
            //(*new_node).parent = self;
            ptr::copy(self, (*new_node).parent, 1);
        }
        self.children.push(
            new_node
        );
    }

    fn has_children(&mut self)->bool {
        return !self.children.is_empty();
    }
}

fn build_tree_from_contents(contents : &str) -> TreeNode {
    let mut dummy = TreeNode {parent: ptr::null_mut(), children: Vec::<*mut TreeNode>::new(), size: 0, name:"dummy".to_string()};
    let mut current_level = -1i32;
    let mut parent : *mut TreeNode = &mut dummy;
    for line in contents.lines(){
        unsafe {
            //println!("parent : {:?},{}",parent, (*parent).name);
            println!("parent : {:?}",parent);
        }
        let level = line.find('-').unwrap() as i32;
        //println!("{level}");
        //let mut dash_split = line.split('-');
        //dash_split.next();
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
        
        if level == current_level {
            unsafe {
                parent = (*parent).parent;
                println!("updating parent to {}", (*parent).name);
            }
        }
        else if level < current_level {
            unsafe {
                parent = (*(*parent).parent).parent;
                println!("updating parent to {}", (*parent).name);
            }
        }
        
        let mut new_child = TreeNode {parent: ptr::null_mut(), children: Vec::<*mut TreeNode>::new(), size: sz, name:name.to_string() };
        unsafe {
            print!("{} ", (*parent).name);
            (*parent).add_child(&mut new_child);
            println!("{:?}, {level}, {current_level}",&mut (*parent));
        }
        current_level = level;
        unsafe{
            parent = *(*parent).children.last().unwrap();
            println!(" new_parent: {}", (*parent).name);
        }
    }
    return dummy;
}

fn main() {
    let mut contents = read_file();

    let mut tree = build_tree_from_contents(&contents);
    /*
    let mut tree = TreeNode { parent: ptr::null_mut(), children: Vec::<*mut TreeNode>::new(), size: 0};

    let mut first_child = TreeNode{parent:ptr::null_mut(), children: Vec::<*mut TreeNode>::new(), size:0};
    
    tree.add_child(&mut first_child);    
    
    let mut second_child = TreeNode{parent:ptr::null_mut(), children: Vec::<*mut TreeNode>::new(), size:0};
    
    tree.add_child(&mut second_child);

    
    */
    println!("{:?}",&mut tree);
}