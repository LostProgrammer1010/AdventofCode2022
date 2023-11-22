use std::collections::HashMap;
use std::fs;
pub fn run() {
    let content = fs::read_to_string("rucksack.txt").expect("File was not read correctly");
    let sack = content.split("\n");
    let mut common_letters = Vec::new();
    for line in sack.into_iter() {
        if line != "" {
            let mid = line.len() / 2;
            common_letters.push(find_matching_letter(&line[..mid], &line[mid..]));
        }
    }
    let sum = get_sum_ruck_sack(common_letters);
}

pub fn run1() {
    let content = fs::read_to_string("rucksack.txt").expect("File was not read correctly");
    let binding: Vec<_> = content.split("\n").into_iter().collect::<Vec<_>>();
    let elf_groups: Vec<&[&str]> = binding.chunks(3).collect();
    let mut common_letters = Vec::new();
    for group in elf_groups {
        for letter in group[0].chars() {
            if group[1].contains(letter) && group[2].contains(letter) {
                common_letters.push(letter);
                break;
            }
        }
    }
    println!("{:?}", get_sum_ruck_sack(common_letters));
}

fn get_sum_ruck_sack(common_letters: Vec<char>) -> i32 {
    let values = String::from("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ");
    let mut letter_values: HashMap<char, i32> = HashMap::new();
    let mut num = 1;
    for letter in values.chars() {
        letter_values.insert(letter, num);
        num += 1;
    }
    let mut sum = 0;
    for i in common_letters.into_iter() {
        sum += *letter_values.get(&i).unwrap();
    }
    sum
}

fn find_matching_letter(left: &str, right: &str) -> char {
    let mut common = 'a';
    for letter in left.chars() {
        if right.contains(letter) {
            common = letter;
            break;
        }
    }
    common
}
