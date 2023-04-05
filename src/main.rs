use std::io;

fn main() {
    println!("Sort an array");

    println!("Please input your arrays: ");
    
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let nums: Vec<i32> = input
        .split_whitespace()
        .map(|s| {
            match s.parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Invalid input: {}", s);
                    std::process::exit(1);
                }
            }
        })
        .collect();

    let result = sort_array(nums);

    println!("The sorted array is: {:?}", result);
}

fn sort_array(mut nums: Vec<i32>) -> Vec<i32> {
    fn quick_sort(nums: &mut [i32]) {
        let len = nums.len();
        if len < 2 {
            return;
        }
        let pivot = nums[(len-1) / 2];
        let (mut l, mut r) = (0, len+1);
        let partition_point = loop {
            while { l+=1; nums[l-1] < pivot } {}
            while { r-=1; nums[r-1] > pivot } {}
            if r <= l {
                break r-1;
            }
            nums.swap(l-1, r-1);
        };
        quick_sort(&mut nums[0..=partition_point]);
        quick_sort(&mut nums[partition_point+1..]);
    }
    quick_sort(&mut nums[..]);
    nums
}