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

#[derive(Debug)]
enum Instr{
    Noop,
    Addx,
}

#[derive(Debug)]
struct Instruction {
    instr: Instr,
    value: i32,
    cycles: u32,
}

fn build_program(contents:&str, program:&mut LinkedList<Instruction>){
    for line in contents.lines(){
        let mut space_split = line.trim().split(' ');
        let instr = space_split.next().unwrap();
        let mut second:&str = "0";
        match space_split.next() {
            Some(n) => second=n,
            None=>{},
        }
        let num = second.parse::<i32>().unwrap();
        let instruction = if instr=="noop" {Instr::Noop} else {Instr::Addx};
        let cycles = if instr=="noop" {1} else {2};
        program.push_back(
            Instruction{
                instr: instruction,
                value: num,
                cycles: cycles,
            }
        );
    }
}

fn execute(instr:&mut Instruction, x:&mut i32) {
    match &instr.instr {
        Instr::Noop => {},
        Instr::Addx => {*x += instr.value},
    }
}

fn run_program(program:&mut LinkedList<Instruction>, x:&mut i32){
    let mut cycle = 1;
    let mut signal_strength = 0;
    let mut display_list = LinkedList::<char>::new();
    while !program.is_empty() {
        {
            let pixel_pos = cycle%40 - 1;
            if pixel_pos >= *x-1 && pixel_pos <= *x+1 {display_list.push_back('#')} else {display_list.push_back('.');}
            if cycle%40 == 0 {display_list.push_back('\n')};
        }
        match cycle {
            20 | 60 | 100 | 140 | 180 | 220 => {
                println!("cycle: {cycle}, x: {}",*x);
                signal_strength += cycle * (*x);
            },
            _ => {},
        }
        
        let mut instr = program.front_mut().unwrap();
        instr.cycles -= 1;
        if instr.cycles == 0 {
            execute(&mut instr, x);
            program.pop_front();
        }
        cycle += 1;
    }
    println!("signal_strength = {signal_strength}"); //part 1
    let display: String = display_list.into_iter().collect();
    println!("{}", display);
}

fn main() {
    let mut contents = read_file();


    let mut program = LinkedList::<Instruction>::new();

    build_program(&contents, &mut program);

    print!("{:?}", program);

    let mut x = 1i32;

    run_program(&mut program, &mut x)
}
