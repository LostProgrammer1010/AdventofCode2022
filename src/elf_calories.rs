use std::fs;

pub fn elves_most_colories() {
    let content = fs::read_to_string("elves_calories.txt").expect("File was not read correctly");
    let each_elf_colories = content.split("\n\n");
    let mut all_elf_calories: Vec<i32> = Vec::new();
    for elf in each_elf_colories {
        let mut sum = 0;
        for num in elf.split("\n") {
            let elf_calorie: i32 = match num.parse() {
                Ok(x) => x,
                Err(_) => 0,
            };
            sum += elf_calorie;
        }
        match all_elf_calories.is_empty() {
            true => all_elf_calories.push(sum),
            false => sort_all_elf_calories(&mut all_elf_calories, sum),
        }
    }
    let end = all_elf_calories.len() - 1;
    println!(
        "{}",
        all_elf_calories[end] + all_elf_calories[end - 1] + all_elf_calories[end - 2]
    )
}

fn sort_all_elf_calories(all_elf_calories: &mut Vec<i32>, sum: i32) {
    let mut found = false;
    for i in 0..all_elf_calories.len() {
        if sum < all_elf_calories[i] {
            all_elf_calories.insert(i, sum);
            found = true;
            break;
        }
    }
    if !found {
        all_elf_calories.push(sum);
    }
}
