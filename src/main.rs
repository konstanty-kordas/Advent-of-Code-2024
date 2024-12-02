use std::fs;

fn main() {
    // day1();
}

fn day1() {
    let file_path = "data/01_input.txt";
    println!("In file {file_path}");

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let lines = contents.split('\n');
    let mut list1: Vec<i32> = vec![];
    let mut list2: Vec<i32> = vec![];

    for line in lines {
        let nums: Vec<&str> = line.split("   ").collect();
        // println!("{}", nums.len());
        list1.push(nums[0].parse().unwrap());
        list2.push(nums[1].parse().unwrap());
        // println!("NUM1: {} ", nums[0]);
        // println!("NUM2: {} ", nums[1]);
    }
    list1.sort_unstable();
    list2.sort_unstable();
    let mut res2: i32 = 0;
    let mut res1: i32 = 0;
    for i in 0..list1.len() {
        res1 += (list1[i]- list2[i]).abs();
        res2 += list1[i] * (list2.iter().filter(|&n| *n == list1[i]).count()) as i32;
    }
    println!("Result 1 {}", res1);  
    println!("Result 2 {}", res2);
}
