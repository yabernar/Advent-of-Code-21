use std::fs;

fn read_input() -> String{
    fs::read_to_string("input").expect("Something went wrong")
}

fn main(){
    let input = read_input();
    let split = input.split("\n");
    let mut measurements = Vec::new();
    for s in split {
        if s != "" {
            let current : u32 = s.parse().unwrap();
            measurements.push(current);
        }
    }

    let mut single_count = 0;
    for i in 1..measurements.len(){
        if measurements[i] > measurements[i-1] {
            single_count += 1
        }
    }

    let mut three_count = 0;
    for i in 1..measurements.len()-2{
        if measurements[i+2] > measurements[i-1] {
            three_count += 1
        }
    }

    println!("Single count : {}\nThree count : {}", single_count, three_count);
}

