use std::fs::File;
use std::io::{self, BufRead};
use regex::Regex;
use substring::Substring;
use std::collections::HashSet;

pub fn part1() -> usize {
    return partn();
}

pub fn part2() -> usize {
    return partn();
}

fn partn() -> usize {
    let file = File::open("data/d045.txt").unwrap();
    let mut buf = io::BufReader::new(file);

    let mut highest = 0;

    loop {
	let mut line = String::new();
	if 0 == buf.read_line(&mut line).unwrap() {
	    return highest;
	}

	let row = rown(line.substring(0, 8));
	let col = coln(line.substring(8, 11));
	
	
    }
}

fn rown(specStr: &str) -> usize {
    let mut row = 0;
    let mut step = 128;

    // let mut front = 0;
    // let mut back = 128;

    for ch in specStr.chars() {
	step = step / 2;
	match ch {
	    'F' => row = row - step,
	    'B' => row = row + step,
	    // 'B' => front = front + step,
	    // 'F' => back = back - step,
	    _ => {},
	}
    }

    return row;
}
