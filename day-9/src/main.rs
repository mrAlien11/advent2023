use std::fs;

fn main() {
    let reader = fs::read_to_string("input.txt").expect("couldn't read");
    // let mut lines = vec![];
    let mut sum = 0;
    reader.lines().collect::<Vec<_>>().into_iter().for_each(|line| {
        let a = line.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<_>>();
        let mut b: Vec<Vec<i32>> = vec![];
        let mut c = vec![];
        c.push(a.clone());
        let mut i = 1;
        let mut  len = a.len();
        let mut temp = vec![];
        let mut j = 0;
        let mut good = false;
        loop {
            if i == len {
                b.push(temp.clone());
                c.push(temp.clone());
                j += 1;
                len = temp.len();
                temp.clear();
                good = true;
                i = 1;
            }
            if good && check_zeroes(b[j - 1].clone()) {
                break;
            }
            if b.is_empty() {
                temp.push(a[i] - a[i - 1]);
            } else {
                good = false;
                temp.push(b[j - 1][i] - b[j - 1][i - 1]);
            }
            i += 1;
        }
        c.reverse();
        for k in 0..c.len() {
            let d = c.clone();
            if check_zeroes(d[k].clone()) {
                c[k].insert(0, 0);
            } else {
                c[k].insert(0,d[k][0] - d[k - 1][0]);
            }
        }
        sum += c.last().unwrap()[0];
    });
    println!("{}", sum);
}

fn check_zeroes(b: Vec<i32>) -> bool {
    let mut yes = true;
    b.into_iter().for_each(|x| {
        if x != 0 {
            yes = false;
        }
    });
    return yes;
}