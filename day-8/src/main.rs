use std::{fs, collections::HashMap};

fn main() {
    let reader = fs::read_to_string("input.txt").expect("couldn't read");
    let lines = reader.lines().collect::<Vec<_>>();

    let mut table = HashMap::new();
    lines[2..].into_iter().for_each(|x| {
        let (a, b) = x.split_once("=").unwrap();
        table.insert(a.trim(), (b.trim().get(1..4).unwrap(), b.trim().get(6..9).unwrap()));
    });
    let mut i = 0;
    let mut j = 0;
    let mut sum: i64 = 0;
    let mut keys = vec![];
    let mut sums = vec![];
    for key in table.keys() {
        if key.chars().last() == Some('A') {
            keys.push(key.to_owned());
        }
    }
    let mut current_key = keys[0];
    loop {
        if i >= lines[0].chars().count() {
            i = 0;
        }
        if lines[0].chars().nth(i).unwrap() == 'R' {
            current_key = table[current_key].1;
            sum += 1;
        } else {
            current_key = table[current_key].0;
            sum += 1;
        }
        i += 1;
        if current_key.chars().last() == Some('Z') && j < keys.len() - 1 {
            j += 1;
            current_key = keys[j];
            sums.push(sum);
            sum = 0;
            i = 0;
        } else if current_key.chars().last() == Some('Z') && j == keys.len() - 1 {
            sums.push(sum);
            break;
        }
    }
    let mut lcm = 1;
    for m in sums {
        lcm = lcm*m/gcd(lcm, m);
    }
    println!("{:?}", lcm);
}

fn gcd(a: i64, b: i64) -> i64 {
    if a == 0 {
        return b;
    }
    return gcd(b % a, a);
}