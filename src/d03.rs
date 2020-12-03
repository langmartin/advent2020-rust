use std::fs::File;
use std::io::{self, BufRead};

pub fn part1() -> usize {
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
	hp = (hp + 3) % wd;
	vp = vp + 1;

	if vp >= ht {
	    return hits;
	}

	if trees[vp][hp] {
	    hits = hits + 1;
	}
    }
}
