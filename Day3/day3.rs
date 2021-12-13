use std::fs;

fn read_input() -> String{
    fs::read_to_string("input").expect("Something went wrong")
}

fn main(){
    let input = read_input();
    let split = input.split("\n");
    let mut bit_nbrs = vec![0; 12];
    let mut total_count = 0;

    for s in split {
        if s != "" {
            for i in 0..s.len() {
                if s.chars().nth(i).unwrap() == '1' {
                    bit_nbrs[i] += 1;
                }
            }
            total_count += 1
        }
    }

    let mut gamma_bin = String::new();
    for i in 0..bit_nbrs.len() {
        if bit_nbrs[i] > total_count/2 {
            gamma_bin.push('1');
        } else {
            gamma_bin.push('0');
        }
    }

    println!("{}", gamma_bin);
    let gamma = u32::from_str_radix(&gamma_bin, 2).unwrap();
    let epsilon = 2_u32.pow(12) - 1 - gamma;

    println!("Gamma : {}\nEpsilon : {}\nMultiplied : {}", gamma, epsilon, gamma*epsilon);

    
}

