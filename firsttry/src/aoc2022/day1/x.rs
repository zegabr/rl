use std::fs::File;
use std::io::{BufRead, BufReader};

fn main(){
    let f = File::open("./in").expect("Unable to open file");
    let f = BufReader::new(f);
     let mut a = vec![];

    let bl = "\n";
    let mut curr_sum = 0;
    for line_result in f.lines() {
        let line = line_result.expect("Unable to read line");
        if line == bl || line == ""{
            a.append(curr_sum);
            curr_sum = 0;
            continue;
        }

        curr_sum += line.parse::<i32>().unwrap();
    }

    a.sort();

    for i in a {
        println!("{}",i);
    }
    println!("{}",mx)
}

