use std::{fs, collections::HashMap};

const MULT: i64 = 1000000;

fn main() {
    let table = make_table();
    let mut checked = vec![];
    let mut sum = 0;
    for key in table.keys() {
        for i in 1..(table.len() + 1) as i64 {
            if *key == i || checked.contains(&i) {
                continue;
            } 
            sum += (table[key].0 - table[&i].0).abs() + (table[key].1 - table[&i].1).abs();
            // println!("distance from {} to {} is {}", key, i, (table[key].0 - table[&i].0).abs() + (table[key].1 - table[&i].1).abs());
        }
        checked.push(*key);
    }
    println!("{}", sum);
    // println!("{:?}", table);
}

fn make_table() -> HashMap<i64, (i64, i64)> {
    let mut table = HashMap::new();
    let reader = fs::read_to_string("input.txt").expect("coudln't read");
    let lines = reader.lines().into_iter().map(|x| x.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
    let mut columns = vec![];
    let mut i = 0;
    let mut add = vec![];
    for _j in 0..lines[0].len() {
        add.push('.');
    }
    
    while i < lines.len() {
        for j in 0..lines[i].len() {
            if lines[i][j] == '#' && !columns.contains(&j) {
                columns.push(j);
            }
        }
        i += 1;
    }
    add.clear();
    columns.sort();
    let mut v = 1;
    for j in 0..lines.len() {
        for k in 0..lines[j].len() {
            if lines[j][k] == '#' {
                table.insert(v, (j as i64, k as i64));
                v += 1;
            }
        }
    }
    let mut count = 0;
    let mut passed_x = 0;
    while count < lines.len() {
        if !lines[count].contains(&'#') {
            for key in 1..table.len() as i64 + 1 {
                if table[&key].0 > count as i64 + passed_x {
                    table.insert(key, (table[&key].0 + MULT - 1, table[&key].1));
                }
            }
            passed_x += MULT - 1;
        }
        count += 1;
    }
    let mut passed = 0;
    for j in 1..columns.len() {
        if columns[j] - columns[j-1] > 1 {
            let sum = columns[j] - columns[j-1] - 1;
            // println!("{}", sum);
            for key in 1..table.len() as i64 + 1 {
                if table[&key].1 >= columns[j] as i64 + passed {
                    table.insert(key, (table[&key].0, table[&key].1 + ((sum as i64)*MULT-sum as i64)));
                }
            }
            passed += sum as i64 *MULT - sum as i64;
        }
    }
    return table;
}

fn make_table_1() -> HashMap<i64, (i64, i64)> {
    let mut table = HashMap::new();
    let reader = fs::read_to_string("input2.txt").expect("coudln't read");
    let mut lines = reader.lines().into_iter().map(|x| x.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
    let mut columns = vec![];
    let mut i = 0;
    let mut add = vec![];
    for _j in 0..lines[0].len() {
        add.push('.');
    }
    
    while i < lines.len() {
        for j in 0..lines[i].len() {
            if lines[i][j] == '#' && !columns.contains(&j) {
                columns.push(j);
            }
        }
        if !lines[i].contains(&'#') {
            lines.insert(i, add.clone());
            i += 1;
        }
        i += 1;
    }
    add.clear();
    columns.sort();
    let mut sum = 0;
    for j in 1..columns.len() {
        if columns[j] - columns[j-1] > 1 {
            for k in 0..lines.len() {
                for _k in 1..columns[j] - columns[j-1] {
                    lines[k].insert(columns[j-1] + 1 + sum, '.');
                }
            }
            sum += columns[j] - columns[j-1] - 1;
        }
    }
    let mut v = 1;
    for j in 0..lines.len() {
        for k in 0..lines[j].len() {
            if lines[j][k] == '#' {
                table.insert(v, (j as i64, k as i64));
                v += 1;
            }
        }
    }
    return table;
}