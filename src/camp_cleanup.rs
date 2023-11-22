use std::error::Error;
use std::fs;
pub fn run() {
    println!("Camp clean up here we come!!");
    let overlap = file_parser(String::from("camp_cleanup.txt"));

    match overlap {
        Ok(x) => println!("{x}"),
        Err(a) => eprintln!("{a}"),
    }
}

fn file_parser(file_name: String) -> Result<i32, Box<dyn Error>> {
    let content = fs::read_to_string(file_name)?;
    let elf_pairs = content
        .split("\n")
        .into_iter()
        .filter(|line| *line != "")
        .collect::<Vec<_>>()
        .into_iter()
        .map(|pair| {
            let mut line = pair.split(",").into_iter();
            (line.next().unwrap(), line.next().unwrap())
        })
        .collect::<Vec<_>>()
        .into_iter()
        .map(|seats| {
            let mut first = seats.0.split("-").into_iter();
            let mut second = seats.1.split("-").into_iter();
            (
                (
                    first.next().unwrap().parse::<i32>().unwrap(),
                    first.next().unwrap().parse::<i32>().unwrap(),
                ),
                (
                    second.next().unwrap().parse::<i32>().unwrap(),
                    second.next().unwrap().parse::<i32>().unwrap(),
                ),
            )
        })
        // change for overlap
        .filter(|((a, b), (c, d))| a <= d && c <= b)
        .count();

    Ok(elf_pairs as i32)
}
