use rand::Rng;
use std::io;

fn main() {
    let mut roll_results: Vec<i32> = Vec::new();

    loop {
        let mut roll_count: String = String::new();

        println!("How many times do you want to roll? : ");

        io::stdin()
            .read_line(&mut roll_count)
            .expect("Failed to read line");

        let roll_count: i32 = match roll_count.trim().parse() {
            Ok(num) => num,
            Err(_) => break,
        };

        for _ in 0..roll_count {
            let curr_roll: i32 = rand::rng().random_range(1..=6);
            roll_results.push(curr_roll);
        }

        let freq_list = calc_freq(&roll_results);
        print_histo(freq_list);
    }
}

fn print_histo(freq_list: [i32; 6]) {
    for idx in 0..freq_list.len() {
        print!("{} : ", idx + 1);
        for _ in 0..freq_list[idx] {
            print!("▨");
        }
        println!();
    }
}

fn calc_freq(roll_list: &Vec<i32>) -> [i32; 6] {
    let mut freq_list: [i32; 6] = [0; 6];

    for num in roll_list {
        freq_list[*num as usize - 1] += 1;
    }

    return freq_list;
}
