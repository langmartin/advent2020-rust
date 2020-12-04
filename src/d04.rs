use std::fs::File;
use std::io::{self, BufRead};
use regex::Regex;
use substring::Substring;
use std::collections::HashSet;

pub fn part1() -> usize {
    return partn(false);
}

pub fn part2() -> usize {
    return partn(true);
}

fn is_valid(key: &str, val: &str) -> bool {
    lazy_static! {
	static ref HCL: Regex = Regex::new(r"^#[a-f0-9]+$").unwrap();
	static ref NUM: Regex = Regex::new(r"^[0-9]+$").unwrap();
    }

    let v = match key {
	"byr" => {
	    let n = val.parse().unwrap_or(0);
	    n >= 1920 && n <= 2002
	},
	"iyr" => {
	    let n = val.parse().unwrap_or(0);
	    n >= 2010 && n <= 2020
	},
	"eyr" => {
	    let n = val.parse().unwrap_or(0);
	    n >= 2020 && n <= 2030
	},
	"hgt" => {
	    let l = val.len();
	    if l < 3 { false; }
	    let n = val.substring(0, l-2).parse().unwrap_or(0);
	    // dbg!(key, val, n, val.substring(l-2, l));
	    match val.substring(l-2, l) {
		"cm" => n >= 150 && n <= 193,
		"in" => n >= 59 && n <= 76,
		_ => false,
	    }
	},
	"hcl" => {
	    if 7 != val.len() { false; }
	    // dbg!(key, val, val.len(), HCL.is_match(val));
	    HCL.is_match(val)
	},
	"ecl" => {
	    match val {
		"amb"|"blu"|"brn"|"gry"|"grn"|"hzl"|"oth" => true,
		_ => false,
	    }
	},
	"pid" => {
	    if val.len() != 9 { false; }
	    NUM.is_match(val)
	}
	"cid" => true,
	_ => false,
    };
    
    // if !v {dbg!(key, val);}
    return v;
}

fn partn(validate: bool) -> usize {
    let empty = Regex::new(r"^[\s]*$").unwrap();

    let expected: HashSet<&'static str> =
	["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid",].iter().cloned().collect();

    let file = File::open("data/d04.txt").unwrap();
    let mut buf = io::BufReader::new(file);
    let mut valid = 0;

    loop {
	let mut record = String::new();

	loop {
	    let mut line = String::new();
	    let len = buf.read_line(&mut line).unwrap();

	    if len == 0 ||
		empty.is_match(&line) {

		    let mut found: HashSet<&str> = HashSet::new();
		    let fs: Vec<&str> = record.split_whitespace().collect();
		    for fi in fs {
			let kv: Vec<&str> = fi.split(":").collect();

			if !validate || is_valid(kv[0], kv[1]) {
			    found.insert(kv[0]);
			}
		    }

		    // if record.contains("094a87") {
		    // 	dbg!(&found, &record, expected.is_subset(&found));
		    // }

		    if expected.is_subset(&found) {
			if validate { dbg!(&record); }
			valid = valid + 1;
		    }

		    if len == 0 {
			return valid;
		    }
		    break;
		} else {
		    record = record + &line;
		}
	}
    }
}
