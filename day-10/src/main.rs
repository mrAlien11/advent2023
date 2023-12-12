use std::fs;

fn main() {
    let reader = fs::read_to_string("input2.txt").expect("couldn't read");
    let lines = reader.lines().collect::<Vec<_>>().iter().map(|x| x.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
}