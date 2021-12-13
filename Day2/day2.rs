use std::fs;

fn read_input() -> String{
    fs::read_to_string("input").expect("Something went wrong")
}

fn main(){
    let input = read_input();
    let split = input.split("\n");
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;

    for s in split {
        if s != "" {
            let command = s.split(" ").collect::<Vec<&str>>();
            let number : u32 = command[1].parse().unwrap();
            match command[0] {
                "forward" => {horizontal += number; depth += aim * number;},
                "down" => aim += number,
                "up" => aim -= number,
                _ => println!("Wrong command"),
            }
        }
    }

    println!("Horizontal : {}\nDepth : {}\nMultiplied : {}", horizontal, depth, horizontal*depth);
}

