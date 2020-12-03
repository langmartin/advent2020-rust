use std::fs::File;
use std::io::{self, BufRead};

pub fn part1() -> usize {
    return partn(3, 1);
}

pub fn part2() -> usize {
    return partn(1, 1) *
	partn(3, 1) *
	partn(5, 1) *
	partn(7, 1) *
	partn(1, 2);
}

fn partn(right: usize, down: usize) -> usize {
    let file = File::open("data/d03.txt").unwrap();
    let trees: Vec<Vec<bool>> = io::BufReader::new(file)
	.lines()
	.map(|l| l
	     .unwrap()
	     .chars()
	     .map(|c|
		  match c {
		 '#' => true,
		 _ => false,
	     })
	     .collect()
	)
	.collect();

    let ht = trees.len();
    let wd = trees[0].len();

    let mut hp = 0;
    let mut vp = 0;
    let mut hits = 0;

    loop {
	hp = (hp + right) % wd;
	vp = vp + down;

	if vp >= ht {
	    return hits;
	}

	if trees[vp][hp] {
	    hits = hits + 1;
	}
    }
}
