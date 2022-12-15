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

fn part_1() {
    let mut contents = String::new();
    
    read_file(&mut contents);

    let mut final_score = 0u32;

    for line in contents.lines(){
        let opponent = line.get(0..1).unwrap();
        let player = line.get(2..3).unwrap();
        println!("{}, {}", player, opponent);
        let mut round_score = 0u32;
        match player {
            "X"=>{ //rock
                round_score += 1;
                match opponent {
                    "A" => round_score += 3, //RVR
                    "B" => round_score += 0, //RVP
                    "C" => round_score += 6, //RVS
                    _ => assert!(false),
                }
            }
            "Y"=>{ //paper
                round_score += 2;
                match opponent {
                    "A" => round_score += 6, //PVR
                    "B" => round_score += 3, //PVP
                    "C" => round_score += 0, //PVS
                    _ => assert!(false),
                }
            }
            "Z"=>{ //scissor
                round_score += 3;
                match opponent {
                    "A" => round_score += 0, //SVR
                    "B" => round_score += 6, //SVP
                    "C" => round_score += 3, //SVS
                    _ => assert!(false),
                }
            }
            _ => assert!(false),
        }
        final_score += round_score;
        println!("{round_score}");
    }
    print!("final score: {final_score}");
}

fn part_2() {
    let mut contents = String::new();
    
    read_file(&mut contents);

    let mut final_score = 0u32;

    for line in contents.lines(){
        let opponent = line.get(0..1).unwrap();
        let player = line.get(2..3).unwrap();
        println!("{}, {}", player, opponent);
        let mut round_score = 0u32;
        match opponent {
            "A"=>{ //rock
                match player {
                    "X" => {
                        round_score += 0; //loss
                        round_score += 3; //scissor lose to rock
                    }, //rock - lose
                    "Y" => {
                        round_score += 3; //draw
                        round_score += 1; //rock draw rock
                    }, //rock - draw 
                    "Z" => {
                        round_score += 6; //win
                        round_score += 2; //paper beat rock
                    }, //rock - win 
                    _ => assert!(false),
                }
            }
            "B"=>{ //paper
                match player {
                    "X" => {
                        round_score += 0; //loss
                        round_score += 1; //rock lose to paper
                    }, //paper - lose
                    "Y" => {
                        round_score += 3; //draw
                        round_score += 2; //paper draw paper
                    }, //paper - draw 
                    "Z" => {
                        round_score += 6; //win
                        round_score += 3; //scissors beat paper
                    }, //paper - win 
                    _ => assert!(false),
                }
            }
            "C"=>{ //scissor
                match player {
                    "X" => {
                        round_score += 0; //loss
                        round_score += 2; //paper lose to scissor
                    }, //scissor - lose
                    "Y" => {
                        round_score += 3; //draw
                        round_score += 3; //scissors draw scissors
                    }, //scissor - draw 
                    "Z" => {
                        round_score += 6; //win
                        round_score += 1; //rock beat paper
                    }, //scissor - win 
                    _ => assert!(false),
                }
            }
            _ => assert!(false),
        }
        final_score += round_score;
        println!("{round_score}");
    }
    print!("final score: {final_score}");
}


fn main() {    
    //part_1();
    part_2();


}
