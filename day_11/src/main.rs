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

#[derive(Debug, Clone)]
enum Operation{
    Mul,
    Mulself,
    Add,
    Addself,
    No,
}

#[derive(Debug, Clone)]
struct Function {
    op: Operation,
    val: u64,
}

impl Function {
    fn eval(&self, x: &u64) -> u64 {
        match self.op {
            Operation::Mul => {
                return self.val * *x
            }
            Operation::Add => {
                return self.val + *x
            }
            Operation::No => {return 0}
            Operation::Addself => {
                return *x + *x
            }
            Operation::Mulself => {
                return *x * *x
            }
        }
    }
}

#[derive(Debug, Clone)]
struct Monkey {
    items: LinkedList<u64>,
    expr: Function,
    div: u64,
    if_true: i32,
    if_false: i32,
    inspected: i32,
}

fn parse_monkeys(contents: &str) -> Vec::<Monkey>{
    let mut vec = Vec::<Monkey>::new();
    for line in contents.lines(){
        if line.trim() == "" {continue;}
        if line.starts_with("Monkey") {
            vec.push(
                Monkey{
                    items : LinkedList::<u64>::new(),
                    expr : Function{op: Operation::Add, val: 0u64},
                    div : 1,
                    if_true : 0,
                    if_false : 0,
                    inspected : 0,
                }
            );
            continue;
        }
        let mut colon_split = line.trim().split(':');
        println!("{line}");
        let label = colon_split.next().unwrap().trim();
        let value = colon_split.next().unwrap().trim();
        match label {
            "Starting items" => {
                for item_str in value.split(','){
                    let item = item_str.trim().parse::<i32>().unwrap();
                    vec.last_mut().unwrap().items.push_back(item as u64);
                }
            },
            "Operation" => {
                let mut new = value.split("new = old "); new.next();
                let mut important = new.next().unwrap().split(' ');
                let operation = important.next().unwrap();
                let val = important.next().unwrap();
                println!("operation: {operation} val: {val}");
                match val.parse::<i32>() {
                    Ok(n) => 
                        match operation {
                            "*" => {
                                vec.last_mut().unwrap().expr = Function{
                                    op : Operation::Mul,
                                    val : n as u64,
                                };
                            }
                            "+" => {
                                vec.last_mut().unwrap().expr = Function{
                                    op : Operation::Add,
                                    val : n as u64,
                                };
                            }
                            &_ => {assert!(false);}
                        },
                    Err(..) =>
                        match operation {
                            "*" => {
                                vec.last_mut().unwrap().expr = Function{
                                    op : Operation::Mulself,
                                    val : 0,
                                };
                            }
                            "+" => {
                                vec.last_mut().unwrap().expr = Function{
                                    op : Operation::Addself,
                                    val : 0,
                                };
                            }
                            &_ => {assert!(false);}
                        },
                }
            },
            "Test" => {
                let mut div_by = value.split("divisible by "); div_by.next();
                let by = div_by.next().unwrap().parse::<i32>().unwrap();
                vec.last_mut().unwrap().div = by as u64;
            },
            "If true" => {
                let mut to_throw = value.split("monkey "); to_throw.next();
                let dest = to_throw.next().unwrap().parse::<i32>().unwrap();
                vec.last_mut().unwrap().if_true = dest;
            },
            "If false" => {
                let mut to_throw = value.split("monkey "); to_throw.next();
                let dest = to_throw.next().unwrap().parse::<i32>().unwrap();
                vec.last_mut().unwrap().if_false = dest;
            },
            &_ => {},
        }
    }
    vec
}

fn execute_round(vec: &mut Vec<Monkey>, common_multiple: u64) {
    let mut monkey = 0;
    while monkey < vec.len() {
        //let func = vec[monkey].expr.clone().bind("old").unwrap();
        while !vec[monkey].items.is_empty() {
            let front = vec[monkey].items.front().unwrap();
            let mut new = vec[monkey].expr.eval(front);
            //new /= 3; //part 1
            new %= common_multiple;
            let test = (new%vec[monkey].div)==0;
            let t = vec[monkey].if_true as usize;
            let f = vec[monkey].if_false as usize;
            if test {
                vec[t].items.push_back(new);
            } else {
                vec[f].items.push_back(new);
            }
            vec[monkey].items.pop_front();
            vec[monkey].inspected += 1;
        }
        monkey += 1;
    }
}

fn execute_monkeys(vec: &mut Vec<Monkey>) {
    //let rounds = 20; //part 1
    let mut common_multiple = 1u64;
    for v in vec.iter(){
        if !(common_multiple%v.div==0) {common_multiple*=v.div} else {}
    }
    for v in vec.iter(){
        println!("{}, {}", v.div ,common_multiple%v.div);
    }
    println!("common multiple: {common_multiple}");
    let rounds = 10000;
    for i in 0..rounds {
        execute_round(vec, common_multiple);
        //println!("{:?}",vec);
    }
}

fn calc_part_1(vec: &Vec<Monkey>) -> u64 {
    let mut two_most_active = Vec::<u64>::from([0,0]);
    for m in vec.iter() {
        if m.inspected as u64 > two_most_active[0] {
            two_most_active[0] = m.inspected as u64;
            two_most_active.sort();
        }
    }
    let mut result = 1u64;
    for m in two_most_active.iter() {
        result *= m;
    }
    return result;
}

fn main() {
    let contents = read_file();
    let mut vec = parse_monkeys(&contents);
    //println!("{:?}",vec);
    execute_monkeys(&mut vec);
    let part_1 = calc_part_1(&vec);
    println!("part_1: {part_1}");
}
