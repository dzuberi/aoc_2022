use std::vec::Vec;

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn update_elves(highest_vec : &mut Vec<i32>, current : &mut i32) {
    if *current > highest_vec[0] {
        highest_vec[0] = *current;
        println!("new elf, last had {}", current);
        highest_vec.sort();
    }
    *current = 0;
}

fn main() {
    println!("Input file name:");
    let mut file_name = String::new();
    std::io::stdin().read_line(&mut file_name).expect("Failed to read input");

    file_name = file_name.trim().to_string();

    print!("Reading {}", file_name);

    let contents = std::fs::read_to_string(file_name)
        .expect("Should have been able to read the file");

    print_type_of(&contents);

    let mut highest_vec = vec![0,0,0];
    print_type_of(&highest_vec);
    let mut current_elf = 0i32;
    for line in contents.lines(){
        match line.parse::<i32>() {
            Ok(n) => {
                current_elf += n;
            }
            Err(_error) => {
                update_elves(&mut highest_vec, &mut current_elf);
            }
        }


        println!("{}", line);
    }
    update_elves(&mut highest_vec, &mut current_elf);
    assert_eq!(current_elf, 0);

    println!("best elves had {:?}", highest_vec);
    print!("total {}", highest_vec.iter().sum::<i32>());
}