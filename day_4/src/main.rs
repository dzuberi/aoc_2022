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

//fn print_tuple(tuple: &)
fn contained(first_tuple: &(i32, i32), second_tuple: &(i32, i32)) -> bool {

    let first_size = first_tuple.1 - first_tuple.0;
    let second_size = second_tuple.1 - second_tuple.0;

    if first_tuple == second_tuple { return true; }
    else if second_size > first_size {
        if second_tuple.0 > first_tuple.0 { return false; }
        if second_tuple.1 < first_tuple.1 { return false; }
        return true;
    }
    else if first_size > second_size {
        if first_tuple.0 > second_tuple.0 { return false; }
        if first_tuple.1 < second_tuple.1 { return false; }
        return true;
    }

    return false;
}

fn overlap(first_tuple: &(i32, i32), second_tuple: &(i32, i32)) -> bool {
    let lower_tuple = if first_tuple.0 < second_tuple.0 {first_tuple} else {second_tuple};
    let higher_tuple = if first_tuple.1 > second_tuple.1 {first_tuple} else {second_tuple};
    if lower_tuple.1 >= higher_tuple.0 {return true;}
    return false;
}

fn main() {
    let part = 2;

    let mut contents = String::new();
    
    read_file(&mut contents);
    
    let mut num_overlaps = 0u32;

    for line in contents.lines() {
        let first_pair = line.get(0..3).unwrap();
        let second_pair = line.get(4..line.len()).unwrap();
        
        let mut v : Vec<(i32,i32)> = Vec::new();

        for mut tuple_str in line.split(','){
            tuple_str.trim();
            let mut tuple_str_split = tuple_str.split('-');
            let mut tuple = (tuple_str_split.next().unwrap().parse::<i32>().unwrap(), tuple_str_split.next().unwrap().parse::<i32>().unwrap());
            v.push(tuple);
        }

        let first_tuple = v[0];
        let second_tuple = v[1];

        if part == 1 {
            if contained(&first_tuple, &second_tuple) { num_overlaps +=1; }
        }
        if part == 2{
            if overlap(&first_tuple, &second_tuple) { num_overlaps +=1; }
        }

        println!("{:?}, {:?}, {}",first_tuple,second_tuple, num_overlaps);

    }

    print!("{}", num_overlaps);
}
