use std::collections::LinkedList;
use std::collections::HashMap;

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

fn main() {
    let day = 2;

    let mut contents = read_file();

    let mut position = 0;

    let mut last_n = LinkedList::<char>::new();
    let mut last_map = HashMap::<char, u32>::new();
    let mut p = 0;
    for c in contents.chars(){
        let max_len = if day == 1 {4} else {14};
        if last_n.len() == max_len {
            let old_c = last_n.pop_front().unwrap();

            let mut old_e = last_map.entry(old_c)
                .or_insert(0);
            *old_e -= 1;
            if *old_e == 0 { last_map.remove(&old_c); }
        }
        let mut e = last_map.entry(c).or_insert(0);
        *e += 1;
        last_n.push_back(c);

        println!("{c}");
        println!("{:?}",last_map);

        for v in last_map.values(){
            println!("{v}, {p}");
            if (*v != 1) || (p < 3) {
                println!("break");
                position = 0;
                break;
            }
            else {position = p;}
        }
        println!("position: {position}");
        p += 1;
        if position != 0 {
            position += 1;
            break;
        }
    }
    
    print!("{position}");
}
