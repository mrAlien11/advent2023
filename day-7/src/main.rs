use std::fs;

fn main() {
    let reader = fs::read_to_string("input.txt").expect("couldn't read");
    let mut hands = vec![];
    reader.lines().collect::<Vec<_>>().into_iter().for_each(|x| {
        let (a, b) = x.split_once(" ").unwrap();
        let mut temp = (0, vec![], [1, 1, 1, 1, 1], 0);
        a.chars().into_iter().for_each(|y| {
            match y {
                'T' => temp.1.push(11),
                'J' => temp.1.push(1),
                'Q' => temp.1.push(13),
                'K' => temp.1.push(14),
                'A' => temp.1.push(15),
                _ => temp.1.push(y.to_digit(10).unwrap() as i32)
            }
        });
        temp.0 = b.parse::<i32>().unwrap();
        let mut checked = vec![];
        for i in 0..temp.1.len() {
            let compare = temp.1[i];
            if !checked.contains(&i) {
                if compare == 1 {
                    temp.2[i] = 10;
                }
                for j in 0..temp.1.len() {
                    if i != j {
                        if compare == temp.1[j] && compare != 1 {
                            temp.2[i] += 1;
                            checked.push(j);
                        } else if compare == temp.1[j] && compare == 1 {
                            temp.2[i] += 10;
                            checked.push(j);
                        }
                    }
                }
            } else {
                temp.2[i] = 0;
            }
        }
        temp.2.sort();
        temp.2.reverse();
        if temp.2[0] > 5 {
            temp.2[1] += temp.2[0] / 10;
            temp.2[0] = 0;
        }
        temp.2.sort();
        temp.2.reverse();
        if temp.2[0] == 5 {
            temp.3 = 6;
        } else if temp.2[0] == 4 {
            temp.3 = 5;
        } else if temp.2[0] == 3 {
            if temp.2[1] == 2 {
                temp.3 = 4;
            } else {
                temp.3 = 3;
            }
        } else if temp.2[0] == 2 {
            if temp.2[1] == 2 {
                temp.3 = 2;
            } else {
                temp.3 = 1;
            }
        } else {
            temp.3 = 0;
        }
        hands.push(temp);
    });
    for _i in 0..hands.len() {
        for j in 0..hands.len() - 1 {
            if hands[j+1].3 < hands[j].3 {
                hands.swap(j, j+1);
            }
        }
    }
    hands.reverse();
    for _i in 0..hands.len() {
        let mut j = 0;
        while j < hands.len() - 1{
            if hands[j].3 == hands[j+1].3 {
                let mut done = false;
                for k in 0..5 {
                    if hands[j].1[k] < hands[j+1].1[k] && !done {
                        hands.swap(j, j+1);
                        done = true;
                    } else if hands[j].1[k] > hands[j+1].1[k] && !done {
                        done = true;
                    }
                }
            }
            j += 1;
        }
    }
    let mut result = 0;
    for i in 0..hands.len() {
        result += hands[i].0 * (hands.len() - i) as i32;
    }
    println!("{}", result);
}

fn part1() {
    let reader = fs::read_to_string("input.txt").expect("couldn't read");
    let mut hands = vec![];
    reader.lines().collect::<Vec<_>>().into_iter().for_each(|x| {
        let (a, b) = x.split_once(" ").unwrap();
        let mut temp = (0, vec![], [1, 1, 1, 1, 1], 0);
        a.chars().into_iter().for_each(|y| {
            match y {
                'T' => temp.1.push(11),
                'J' => temp.1.push(12),
                'Q' => temp.1.push(13),
                'K' => temp.1.push(14),
                'A' => temp.1.push(15),
                _ => temp.1.push(y.to_digit(10).unwrap() as i32)
            }
        });
        temp.0 = b.parse::<i32>().unwrap();
        let mut checked = vec![];
        for i in 0..temp.1.len() {
            let compare = temp.1[i];
            if !checked.contains(&i) {
                for j in 0..temp.1.len() {
                    if i != j {
                        if compare == temp.1[j] {
                            temp.2[i] += 1;
                            checked.push(j);
                        }
                    }
                }
            } else {
                temp.2[i] = 0;
            }
        }
        temp.2.sort();
        temp.2.reverse();
        if temp.2[0] == 5 {
            temp.3 = 6;
        } else if temp.2[0] == 4 {
            temp.3 = 5;
        } else if temp.2[0] == 3 {
            if temp.2[1] == 2 {
                temp.3 = 4;
            } else {
                temp.3 = 3;
            }
        } else if temp.2[0] == 2 {
            if temp.2[1] == 2 {
                temp.3 = 2;
            } else {
                temp.3 = 1;
            }
        } else {
            temp.3 = 0;
        }
        hands.push(temp);
    });
    for _i in 0..hands.len() {
        for j in 0..hands.len() - 1 {
            if hands[j+1].3 < hands[j].3 {
                hands.swap(j, j+1);
            }
        }
    }
    hands.reverse();
    for _i in 0..hands.len() {
        let mut j = 0;
        while j < hands.len() - 1{
            if hands[j].3 == hands[j+1].3 {
                let mut done = false;
                for k in 0..5 {
                    if hands[j].1[k] < hands[j+1].1[k] && !done {
                        hands.swap(j, j+1);
                        done = true;
                    } else if hands[j].1[k] > hands[j+1].1[k] && !done {
                        done = true;
                    }
                }
            }
            j += 1;
        }
    }
    let mut result = 0;
    for i in 0..hands.len() {
        result += hands[i].0 * (hands.len() - i) as i32;
    }
    println!("{}", result);
}