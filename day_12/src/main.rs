use std::vec::Vec;
use std::collections::VecDeque;

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

fn build_vec(contents: &str) -> (Vec<Vec<char>>, (usize, usize), (usize, usize)) {
    let mut vec = Vec::<Vec<char>>::new();
    let mut start_index = (0,0);
    let mut end_index = (0,0);
    for line in contents.lines(){
        vec.push(Vec::<char>::new());
        for c in line.chars() {
            vec.last_mut().unwrap().push(c);
            if c == 'S'{
                start_index.0 = vec.len()-1;
                start_index.1 = vec.last().unwrap().len()-1;
            }
            if c == 'E'{
                end_index.0 = vec.len()-1;
                end_index.1 = vec.last().unwrap().len()-1;
            }
        }
    }
    return (vec, start_index, end_index);
}

fn get_elevation(c: &char) -> i32{
    let new_c = if *c == 'E' {'z'} else if *c == 'S' {'a'} else {*c};
    new_c as i32 - 'a' as i32
}

fn breadth_first_search(start: (usize, usize), end: (usize, usize), vec: &Vec<Vec<char>>) -> Option<usize> {
    let x = start.0;
    let y = start.1;
    let mut visited = vec![vec![false;vec[0].len()]; vec.len()];
    let mut visited_nodes = 0;
    let mut q = VecDeque::<((usize, usize),usize)>::new();            
    q.push_back((start.clone(),0));
                
    while let Some((position, length)) = q.pop_front() {
        if position == end {return Some(length)}
        else{
            let (x,y) = position;
            visited[x][y] = true;
            visited_nodes += 1;
            if (visited_nodes % 100) == 0 {println!("visited {visited_nodes}")}
            IntoIterator::into_iter([(-1,0), (1,0), (0,-1), (0,1)]).flat_map(|(dx, dy)| 
                {
                    Some (
                        (
                            TryInto::<usize>::try_into((x as i32)+dx).ok()?,
                            TryInto::<usize>::try_into((y as i32)+dy).ok()?
                        )
                    )
                } )
                .filter(|&(new_x, new_y)| new_x < vec.len() && new_y < vec[0].len())
                // .filter(|&(new_x, new_y)| !visited[new_x][new_y].clone())
                .filter(|&(new_x, new_y)| (get_elevation(&vec[new_x][new_y]) - get_elevation(&vec[x][y]) <= 1))
                .for_each(|(new_x, new_y)|{
                        if !visited[new_x][new_y] {q.push_back(((new_x, new_y), length+1))}
                        visited[new_x][new_y] = true;
                        // q.push_back
                    }
                )
 
        }
    }

    return None;
}

fn part_2(vec : &Vec<Vec<char>>, end: (usize, usize)) -> usize{
    let mut result = vec.len() * vec[0].len();
    for (x, v) in vec.iter().enumerate(){
        for (y, c) in v.iter().enumerate() {
            if get_elevation(c) > 0 {continue}
            if let Some(num_steps) = breadth_first_search((x,y), end, vec) {
                if num_steps < result {result = num_steps}
            }
        }
    }
    return result;
}

fn main() {
    let contents = read_file();
    let (mut vec, start_index, end_index) = build_vec(&contents);
    
    println!("{:?}",vec);
    println!("{:?}",start_index);
    println!("{:?}",end_index);
    println!("squared: {}",vec.len().pow(2));
    //let part_1 = get_length_to_path(end_index, 0, &vec);
    //println!("part_1: {part_1}");
    let part_1_opt = breadth_first_search(start_index, end_index, &vec);
    println!("part 1: {:?}",part_1_opt);
    
    let part_2 =part_2(&vec, end_index);
    println!("part 2: {:?}",part_2)
}
