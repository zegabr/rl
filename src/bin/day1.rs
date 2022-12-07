use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let f = File::open("./src/bin/day1_in").expect("Unable to open file");
    let f = BufReader::new(f);
    let mut a = vec![];

    let bl = "\n";
    let mut curr_sum = 0;
    let mut mx = 0;
    for line_result in f.lines() {
        let line = line_result.expect("Unable to read line");
        if line == bl || line == "" {
            a.push(curr_sum);
            mx = i64::max(mx, curr_sum);
            curr_sum = 0;
            continue;
        }

        curr_sum += line.parse::<i64>().unwrap();
    }
    // get the last one
    a.push(curr_sum);
    mx = i64::max(mx, curr_sum);

    println!("{}", mx);
    // ----- part 2 ---
    a.sort();
    // for pos in a.iter() {
    //     println!("ai: {}", pos);
    // }

    let mut top_tree_sum = 0;
    let i = a.len() - 1;
    for pos in i - 2..i + 1 {
        top_tree_sum += a[pos];
    }
    println!("{}", top_tree_sum);
}
