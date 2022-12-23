use std::sync::Arc;
use std::cell::RefCell;

#[derive(Debug)]
pub struct TreeNode<T>{
    pub parent: Option<Arc<RefCell<TreeNode<T>>>>,
    pub children: Vec<Arc<RefCell<TreeNode<T>>>>,
    value: T,
}


impl <T> TreeNode <T>{
    pub fn add_child(&mut self, new_child : Arc<RefCell<TreeNode<T>>>){
        self.children.push(
            new_child
        );
    }

    pub fn add_child_to_self(self_rc : Arc<RefCell<TreeNode<T>>>, new_child : Arc<RefCell<TreeNode<T>>>){
        self_rc.clone().borrow_mut().children.push(
            new_child.clone()
        );
        new_child.borrow_mut().parent = Some(self_rc.clone());
    }

    pub fn has_children(&mut self)->bool {
        return !self.children.is_empty();
    }
    
    pub fn get_new_tree(value: T) -> Arc<RefCell<TreeNode<T>>> {
        let dummy = Arc::new( 
            RefCell::new(
                TreeNode {
                    parent: None, 
                    children: Vec::<Arc<RefCell<TreeNode<T>>>>::new(), 
                    value: value,
                } ) );
        return dummy;
    }
}

impl <T : std::fmt::Debug > TreeNode <T>{
    pub fn print_self(&self) {
        println!("-------------------tree--------------------\n");
        println!("{:?}", self.value);
        for child in self.children.iter() {
            Self::print_tree(child, 1);
        }
        println!("");
    }

    pub fn print_tree(head: &Arc<RefCell<TreeNode<T>>>, level: u32) {
        let value = &head.borrow().value;
        for _ in 0..level{
            print!(" ");
        }
        println!("{:?}", value);
        for child in head.borrow().children.iter(){
            Self::print_tree(child, level+1);
        }
    }
    
    pub fn print_whole_tree(&self) {
        if let Some(parent) = &self.parent {
            parent.borrow().print_whole_tree();
        }
        else {self.print_self();}
    }
}