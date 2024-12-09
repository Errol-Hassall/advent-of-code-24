use std::fs::File;
use std::io;
use std::io::{Read};

fn filename_to_string(s: &str) -> io::Result<String> {
    let mut file = File::open(s)?;
    let mut s = String::new();
    file.read_to_string(&mut s)?;
    Ok(s)
}

fn words_by_line(s: &str) -> Vec<Vec<&str>> {
    s.lines().map(|line| {
        line.split_whitespace().collect()
    }).collect()
}

pub fn puzzle_one() -> io::Result<i32> {
    let whole_file = filename_to_string("./src/list.txt")?;
    let wbyl = words_by_line(&whole_file);

    let mut list_one: Vec<i32> = vec![];
    let mut list_two: Vec<i32> = vec![];
    let mut list_diff = vec![];

    for line in wbyl {
        list_one.push(line[0].parse().unwrap());
        list_two.push(line[1].parse().unwrap());
    }

    list_one.sort();
    list_two.sort();

    let mut index = 0;

    while index < list_one.len() {
        let diff = list_two[index] - list_one[index];

        list_diff.push(diff.abs());
        index +=1;
    }

    let mut total = 0;

    for element in list_diff {
        total += element
    }

    Ok(total)
}
