use std::collections::HashMap;
use std::collections::LinkedList;

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

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone)]
struct Position {
    horiz: i32,
    vert: i32,
}

impl Position {
    fn is_touching(&self, other: &Position) -> bool {
        ((self.horiz - other.horiz).abs() <= 1) && ((self.vert - other.vert).abs() <= 1)
    }
}

fn move_tail(h_pos: &Position, t_pos: &mut Position) {
    if h_pos.is_touching(t_pos) {return};
    let horiz_diff = h_pos.horiz - t_pos.horiz;
    let horiz_mag = horiz_diff.abs();
    let vert_diff = h_pos.vert - t_pos.vert;
    let vert_mag = vert_diff.abs();
    if vert_mag >= 1 {
        let vert_dir = vert_diff/vert_mag;
        t_pos.vert += vert_dir;
    }
    if horiz_mag >= 1 {
        let horiz_dir = horiz_diff/horiz_mag;
        t_pos.horiz += horiz_dir;
    }
}

fn process_positions(contents:&str, visited_positions:&mut HashMap<Position,bool>) {
    let total_length = 10; //2 for part 1, 10 for part 2
    let mut h_pos = Position{horiz:0,vert:0};
    let mut t_pos = Position{horiz:0,vert:0};
    let mut rope = Vec::<Position>::new();
    for _ in 0..total_length {rope.push(Position{horiz:0, vert:0});}
    visited_positions.insert(t_pos.clone(), true);
    for line in contents.lines(){
        let mut split = line.split(' ');
        let dir = split.next().unwrap();
        let mut amount = split.next().unwrap().parse::<i32>().unwrap();
        while amount > 0 {
            match dir {
                "R" => {
                    rope[0].horiz += 1;
                },
                "L" => {
                    rope[0].horiz -= 1;
                }
                "U" => {
                    rope[0].vert += 1;
                }
                "D" => {
                    rope[0].vert -= 1;
                }
                &_ => {assert!(false);}
            }
            let mut curr_index = 0;
            let mut next_index = 1;
            while next_index < rope.len() {
                move_tail(&rope.get(curr_index).unwrap().clone(),&mut rope.get_mut(next_index).unwrap());
                curr_index += 1;
                next_index += 1;
            }
            visited_positions.insert(rope[rope.len()-1].clone(), true);
            amount-=1;
        }
    }
}

fn main() {
    let mut contents = read_file();

    let mut visited_positions = HashMap::<Position, bool>::new();

    process_positions(&contents, &mut visited_positions);

    let part1 = visited_positions.keys().len();
    println!("{}",part1);

    let mut ll = LinkedList::<i32>::new();
    ll.push_back(0);
    ll.push_back(1);
    ll.push_back(2);
    ll.push_back(3);
    let mut curr_iter = ll.iter();
    let mut next_iter = ll.iter();
    next_iter.next();
    for next in next_iter {
        let curr = curr_iter.next().unwrap();
        println!("{curr}, {next}");
    }
}
