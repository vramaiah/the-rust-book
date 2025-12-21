use std::{collections::HashMap, f64, i32};

fn mode(v: &Vec<i32>) -> Option<i32> {
    let map = {
        let mut map = HashMap::new();

        for elem in v {
            let count = map.entry(*elem).or_insert(0);
            *count += 1;
        }
        map
    };

    let mut greatest_entry = 0;
    let mut greatest_count = 0;
    for (entry, count) in &map {
        if count > &greatest_count {
            greatest_entry = *entry;
            greatest_count = *count;
        }
    }

    if greatest_count != 1 {
        Some(greatest_entry)
    } else {
        None
    }
}

fn median(v: &Vec<i32>) -> f64 {
    let mut v = v.clone();
    v.sort();
    let length = v.len();
    if length % 2 == 0 {
        v[length / 2] as f64
    }
    else {
        ((v[length / 2] as f64) + (v[(length / 2) + 1] as f64)) / 2.0
    }
}

fn main() {
    let vec = vec![1, 2, 3, 5];
    let mode = mode(&vec);
    let median = median(&vec);

    match mode {
       Some(n) => println!("mode is {n}"),
       None => println!("no mode")
    }

    println!("median is {median}");
}
