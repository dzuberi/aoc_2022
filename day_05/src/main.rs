use std::collections::LinkedList;
use std::vec::Vec;

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn read_file(file_contents: &mut String) {
    println!("Input file name:");
    let mut file_name = String::new();
    std::io::stdin().read_line(&mut file_name).expect("Failed to read input");

    file_name = file_name.trim().to_string();

    print!("Reading {}", file_name);

    *file_contents = std::fs::read_to_string(file_name)
        .expect("Should have been able to read the file");

    print_type_of(file_contents);
}

fn get_stacks_from_contents(contents: &mut String) -> Vec<LinkedList<char>>{
    let first_line_len = contents.split('\n').next().unwrap().len();
    let num_stacks = (first_line_len + 1)/4;

    let mut vec: Vec<LinkedList<char>> = Vec::new();

    println!("{num_stacks}");

    for _i in 0..num_stacks{
        vec.push(LinkedList::new());
    }

    for line in contents.lines() {
        match line.trim().get(0..1).unwrap().parse::<i32>() {
            Ok(..) => break,
            Err(..) => {}
        }

        for i in 0..num_stacks{
            let str_index = 1 + i*4;
            let slice = line.get(str_index..str_index+1).unwrap();
            if slice.trim() == "" {continue;}
            vec[i].push_back(slice.chars().next().unwrap());
        }
    }
    return vec;
}

fn process_moves_p1(moves: &str, stacks: &mut Vec<LinkedList<char>>){
    for m in moves.lines(){
        println!("{m}");
        let mut split_space = m.split(' ');
        assert_eq!(split_space.next().unwrap(),"move");
        let n = split_space.next().unwrap().parse::<i32>().unwrap();
        assert_eq!(split_space.next().unwrap(),"from");
        let from = split_space.next().unwrap().parse::<i32>().unwrap() - 1;
        assert_eq!(split_space.next().unwrap(),"to");
        let to = split_space.next().unwrap().parse::<i32>().unwrap() - 1;
        println!("{n}, {from}, {to}");
        for i in 0..n{
            let popped = stacks[from as usize].pop_front().unwrap();
            stacks[to as usize].push_front(popped);
        }
        println!("{:?}", stacks);
    }
}

fn process_moves_p2(moves: &str, stacks: &mut Vec<LinkedList<char>>){ //this is ugly because i should have used push_back but im not fixing it!
    for m in moves.lines(){
        println!("{m}");
        let mut split_space = m.split(' ');
        assert_eq!(split_space.next().unwrap(),"move");
        let n = split_space.next().unwrap().parse::<i32>().unwrap();
        assert_eq!(split_space.next().unwrap(),"from");
        let from = split_space.next().unwrap().parse::<i32>().unwrap() - 1;
        assert_eq!(split_space.next().unwrap(),"to");
        let to = split_space.next().unwrap().parse::<i32>().unwrap() - 1;
        println!("{:?}", stacks);
        let mut popped_list = LinkedList::<char>::new();
        for i in 0..n{
            let popped = stacks[from as usize].pop_front().unwrap();
            popped_list.push_back(popped);
        }
        println!("{:?}", popped_list);
        popped_list.append(&mut stacks[to as usize]);
        stacks[to as usize] = popped_list.clone();
        println!("{:?}", stacks);
    }
}

fn main() {

    let part = 2;

    let mut contents = String::new();

    read_file(&mut contents);

    let mut stacks = get_stacks_from_contents(&mut contents);
    println!("{:?}",stacks);

    let moves = contents.split("\r\n\r\n").nth(1).unwrap().trim_start();

    if part == 1{
        process_moves_p1(&moves, &mut stacks);
    }

    if part == 2{
        process_moves_p2(&moves, &mut stacks);
    }

    for s in stacks.iter() {
        print!("{}",s.front().unwrap());
    }
}
