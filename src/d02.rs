use std::fs::File;
use std::io::{self, BufRead};

fn valid(line:&str) -> bool {
    let policy_passwd:Vec<&str> = line.split(":").take(2).collect();
    let policy = policy_passwd[0];
    let passwd = policy_passwd[1];

    let policy_fs:Vec<&str> = policy.split(" ").take(2).collect();
    let range:Vec<&str> = policy_fs[0].split("-").collect();

    let check = &policy_fs[1].chars().next().unwrap();
    let start = range[0].parse().unwrap();
    let end = range[1].parse().unwrap();

    let count = passwd.chars().filter(|c| c == check).count();

    return count >= start && count <= end;
}

pub fn part1() -> usize {
    let file = File::open("data/d02.txt").unwrap();
    return io::BufReader::new(file)
	.lines()
	.map(|l| l.unwrap())
	.filter(|l| valid(l))
	.count();
}
