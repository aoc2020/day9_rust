use std::fs::File;
use std::io::{self, Lines, BufReader, BufRead};
use std::path::Path;

fn is_valid(numbers:&Vec<i64>, pre: usize, curr: usize) -> bool {
    let slice = &numbers[curr - pre..curr];
    for x in slice {
        for y in slice {
            if (x != y) && (x + y == numbers[curr])   {
                return true
            }
        }
    }
    return false;

    // let candidates : Vec<(i64,i64)> = slice.into_iter()
    //     .map(|x| slice.into_iter().map(|y| (x.clone(),y.clone())))
    //     .flatten().collect();
    // false
}

fn find_invalid(numbers:&Vec<i64>, pre: usize, curr: usize) -> i64 {
    for i in curr..numbers.len() {
        if !is_valid(numbers, pre, i) {
            return numbers[i].clone();
        }
    }
    panic!("No invalid numbers found");
}

fn find_range_last(numbers:&Vec<i64>,first:usize,target: i64) -> Option<usize> {
    let mut sum = numbers[first];
    for last in first+1..numbers.len() {
        sum += numbers[last];
        if sum > target {
            return None
        }
        if sum == target {
            return Some(last.clone());
        }
    }
    return None
}

fn find_range(numbers:&Vec<i64>, target:i64) -> (usize,usize) {
    for first in 0..numbers.len() {
        if let Some(last) = find_range_last(&numbers,first,target) {
            return (first,last);
        }
    }
    panic!("No range found");
}

fn find_minmax(numbers:&Vec<i64>,first:usize,last:usize) -> (i64,i64) {
    let mut min = numbers[first];
    let mut max = numbers[first];
    for i in first+1..last {
        if numbers[i] > max {
            max = numbers[i].clone();
        }
        if numbers[i] < min {
            min = numbers[i].clone();
        }
    }
    (min,max)
}

fn solve_tasks(numbers:Vec<i64>, pre: usize) {
    let invalid = find_invalid(&numbers, pre,pre);
    println!("Answer 1: {}", invalid);
    let (first,last) = find_range(&numbers, invalid);
    let (min,max) = find_minmax(&numbers, first,last);
    println!("Answer 2: [{},{}] => [{}+{}] = {}",first,last,min,max,min+max);
}

fn main() {
    if let Ok(lines) = read_lines("input.txt") {
        let input : Vec<i64> = lines.into_iter().map(|s| s.unwrap())
            .map(|s|s.parse().unwrap()).collect();
        solve_tasks(input,25);
    } else {
        println!("Error reading file");
    }
}

fn read_lines<P>(filename: P) -> io::Result<Lines<BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}