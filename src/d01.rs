use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn numbers(filename: &str) -> Vec<i32> {
    let mut nums = Vec::new();

    if let Ok(lines) = read_lines(filename) {
	for line in lines {
	    if let Ok(s) = line {
		let n = s.parse::<i32>().unwrap();
		nums.push(n);
	    }
	}
    }

    return nums;
}

pub fn part1() -> i32 {
    let nums = numbers("data/d01.txt");

    for n in &nums {
	for m in &nums {
	    if n + m  == 2020 {
		return n * m;
	    }
	}
    }

    return 0;
}

pub fn part2() -> i32 {
    let nums = numbers("data/d01.txt");

    for n in &nums {
	for m in &nums {
	    for o in &nums {
		if n + m + o == 2020 {
		    return n * m * o;
		}
	    }
	}
    }

    return 0;
}
