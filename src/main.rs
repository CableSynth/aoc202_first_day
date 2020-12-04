use std::collections::HashSet;
use std::fs;

fn main() {
    let mut hash1: HashSet<i64> = HashSet::new();
    let filename = "/home/caleb/codestuff/advent2020/FirstDay/src/input.txt";
    let data =  fs::read_to_string(filename).expect("error");
    let data_split: Vec<&str> = data.split_ascii_whitespace().collect();
    for number in data_split {
        let num = number.parse::<i64>().expect("Bad Value");
        let val = 2020_i64 - num;
        if hash1.contains(&val) {
            println!("found {}", val * num);
            break;
        }
        hash1.insert(num);
    }

    let data_split: Vec<&str> = data.split_ascii_whitespace().collect();
    let mut hash2: HashSet<i64> = HashSet::new();
    for i in 0..(data_split.len() -1) {
        let first_num = data_split[i].parse::<i64>().expect("Bad Value");
        let current_sum = 2020_i64 - first_num;
        for j in (i+1)..data_split.len() {
            let second_num = data_split[j].parse::<i64>().expect("Bad Value");
            if hash2.contains(&(current_sum - second_num)) {
                println!("found {}", (current_sum - second_num)*(first_num * second_num));
                return;
            }
            hash2.insert(second_num);
        }
    }
}
