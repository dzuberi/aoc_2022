use std::collections::HashMap;

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

fn get_int_from_char(c: &char) -> u32 {
    let mut retval = 0;
    if *c as u32 >= 'a' as u32 && *c as u32 <='z' as u32{
        retval = *c as u32 - 'a' as u32 + 1;
    }
    else {
        retval = *c as u32 - 'A' as u32 + 27;
    }
    return retval;
}

fn part_1(){
    let mut contents = String::new();
    
    read_file(&mut contents);

    let mut sum = 0;

    for line in contents.lines(){
        let length = line.len() as usize;
        let half = length/2 as usize;
        let first_half = line.get(0..half).unwrap();
        let second_half = line.get(half..length).unwrap();
        println!("{first_half} {second_half}");
        
        let mut first_map = HashMap::new();

        for c in first_half.chars(){
            first_map.insert(c, true);
        }

        let mut num = 0;

        for c in second_half.chars(){
            if first_map.contains_key(&c){
                num = get_int_from_char(&c);
                println!("{c}, {num}");
                break;
            }
        }
        assert!(num > 0);
        sum += num;
    }
    println!("{}", sum);
}

fn part_2(){
    let mut contents = String::new();
    
    read_file(&mut contents);

    let mut sum = 0;

    let mut i = 0u32;
    let mut char_map = HashMap::new();
    for line in contents.lines(){
        let mut num = 0;
        if (i % 3) == 0 {
            char_map = HashMap::new();
        }

        let mut line_map = HashMap::new();
        for c in line.chars(){
            line_map.entry(c).or_insert(0);
        }

        for (c, val) in line_map.into_iter(){
            let character = char_map.entry(c).or_insert(0);
            *character += 1;
            if (i % 3) == 2 {
                if *character == 3{
                    num = get_int_from_char(&c);
                    println!("{c}, {num}");
                    break;
                }
            }
        }

        if (i % 3) == 2 {
            assert!(num > 0);
            sum += num;
        }
        i+=1;
    }
    println!("{}", sum);
}

fn main() {
    part_2();
}
