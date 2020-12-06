use std::fs::File;
use std::io::{self, BufRead};
use substring::Substring;
use std::cmp::max;

pub fn part1() -> usize {
    return partn();
}

pub fn part2() -> usize {
    return partn();
}

fn partn() -> usize {
    let file = File::open("data/d05.txt").unwrap();
    let buf = io::BufReader::new(file);

    let mut highest = 0;
    for line in buf.lines() {
	if let Ok(line) = line {
	    let row = rown(&line);
	    let col = coln(&line);
	    let id = row * 8 + col;
	    // dbg!(line, row, col, id);
	    highest = max(highest, id);
	}
    }

    return highest;
}

fn rown(line: &str) -> usize {
    return bin(line.substring(0, 7), 128);
}

fn coln(line: &str) -> usize {
    return bin(line.substring(7, 10), 8);
}

fn bin(spec: &str, maximum: usize) -> usize {
    // let mut hi = maximum;
    let mut lo = 0;
    let mut step = maximum;

    for ch in spec.chars() {
	step = step / 2;
	// dbg!(ch, step, lo, hi);
	match ch {
	    'B' | 'R' => lo = lo + step,
	    // 'F' | 'L' => hi = hi - step,
	    _ => {},
	}
    }

    // dbg!(spec, maximum, hi, lo);
    return lo;
}
