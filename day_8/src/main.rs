use std::vec::Vec;

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
struct Vis {
    height: i32,
    vis: bool,
    scenic: i32,
}

impl Default for Vis {
    fn default() -> Vis {
        Vis{
            height: 0,
            vis:false,
            scenic:0,
        }
    }
}

fn set_vis(vec:&mut Vec<Vec<Vis>>) {
    let mut max = -1;
    let size = vec.len();

    //left
    for i in 0..size {
        max = -1;
        for j in 0..size {
            let row = i;
            let col = j;
            if vec[row][col].height > max {
                max = vec[row][col].height;
                vec[row][col].vis = true;
            }
        }
    }

    //right
    for i in 0..size {
        max = -1;
        for j in 0..size {
            let row = i;
            let col = size-1-j;
            if vec[row][col].height > max {
                max = vec[row][col].height;
                vec[row][col].vis = true;
            }
        }
    }

    //top
    for i in 0..size {
        max = -1;
        for j in 0..size {
            let row = j;
            let col = i;
            if vec[row][col].height > max {
                max = vec[row][col].height;
                vec[row][col].vis = true;
            }
        }
    }

    //bottom
    for i in 0..size {
        max = -1;
        for j in 0..size {
            let row = size-1-j;
            let col = i;
            if vec[row][col].height > max {
                max = vec[row][col].height;
                vec[row][col].vis = true;
            }
        }
    }
}

fn count_vis(vec: &Vec<Vec<Vis>>) -> i32 {
    let mut count = 0i32;
    let size = vec.len();
    for i in 0..size {
        for j in 0..size{
            if vec[i][j].vis {count+=1;}
        }
    }
    return count;
}

fn scenic_subroutine(vec: &Vec<Vec<Vis>>, row:i32, col:i32, row_inc:i32, col_inc:i32)->i32{
    let size = vec.len() as i32;
    let mut visible_trees = 0;
    let mut i = row;
    let mut j = col;
    let house_height = vec[row as usize][col as usize].height;
    let mut end = false;
    while !end {
        i += row_inc;
        j += col_inc;
        if i < 0 || j < 0 || i >= size || j >= size {end=true;}
        if end {break;}
        visible_trees += 1;
        let height = vec[i as usize][j as usize].height;
        if height >= house_height {end=true;}
    }
    return visible_trees;
}

fn get_scenic(vec: &Vec<Vec<Vis>>, row:i32, col:i32) -> i32{
    let left = scenic_subroutine(&vec, row, col, 0, -1);
    let right = scenic_subroutine(&vec, row, col, 0, 1);
    let top = scenic_subroutine(&vec, row, col, -1, 0);
    let bottom = scenic_subroutine(&vec, row, col, 1, 0);
    
    return left*right*top*bottom;
}

fn set_scenics(vec: &mut Vec<Vec<Vis>>) -> i32 {
    let size = vec.len();
    let mut max_scenic = 0i32;

    for i in 0..size {
        for j in 0..size{
            let scenic = get_scenic(&vec, i as i32, j as i32);
            vec[i][j].scenic = scenic;
            if scenic > max_scenic { max_scenic = scenic; }
        }
    }
    return max_scenic;
}

fn main() {
    let mut vec = Vec::<Vec<Vis>>::new();
    /*vec.push(Vec::<Vis>::new());
    vec.get_mut(0).unwrap().push(Vis{..Default::default()})
    */
    let contents = read_file();

    for line in contents.lines() {
        vec.push(Vec::<Vis>::new());
        let mut current_row : &mut Vec<Vis> = vec.last_mut().unwrap();
        for c in line.chars(){
            let height = c.to_digit(10).unwrap() as i32;
            current_row.push(Vis{height:height, vis:false, scenic:0});
        }
    }

    assert_eq!(vec.len(), vec.last().unwrap().len());

    set_vis(&mut vec);
    let part_1 = count_vis(&vec);

    let part_2 = set_scenics(&mut vec);
    print!("{part_1}, {part_2}");
}
