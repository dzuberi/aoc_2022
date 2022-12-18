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

fn convert_input(contents: &str) -> String {
    let mut string = String::new();
    let mut level = 0;
    for line in contents.lines(){
        let mut split = line.split(' ');
        let mut cmd = "";
        let first = split.next().unwrap();
        let mut second = "";
        match split.next(){
            Some(s) => second=s,
            None=>{},
        }
        let mut third = "";
        match split.next(){
            Some(s) => third=s,
            None=>{},
        }
        if first.starts_with("$"){
            cmd = second;
        }
        if cmd == "cd" {
            if third == ".."{
                level -= 1;
            }
            else {
                for i in 0..level{
                    string.push_str(" ");
                }
                string.push_str("- ");
                string.push_str(third);
                string.push_str(" (dir)\n");
                level += 1;
            }
            continue;
        }
        match first.parse::<i32>(){
            Ok(s) => {
                for i in 0..level{
                    string.push_str(" ");
                }
                string.push_str("- ");
                string.push_str(second);
                string.push_str(" (file, size=");
                string.push_str(first);
                string.push_str(")\n");
            }
            Err(..) => {}
        }
        println!("first: {first}, second: {second}, third: {third}, level: {level}");
    }
    return string;
}

fn main() {
    let mut contents = read_file();
    let test = convert_input(&contents);
    print!("{test}");
    //process_file_tree_string(&contents);
}