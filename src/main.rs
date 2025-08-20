//to bring HashMaps into scope
use std::collections::HashMap;

fn median_mode(nums: &[i32]) -> Option<(f64, Vec<i32>)>{
    if nums.is_empty(){
        return None;
    }
    let mut sorted = nums.to_vec();
    //sort the list

    sorted.sort_unstable();

    // get the length of the list

    let n = sorted.len();

    let median = if n % 2 == 1{
        sorted[n / 2] as f64
    } else{
        let a = sorted[n / 2 - 1] as f64;
        let b = sorted[n / 2] as f64;
        ( a + b ) / 2.0
    };


    // mode
    // create a hashmap for storing keys and value
    let mut counts = HashMap::new();

    //fill the hashmap with values
    for &v in nums{
        *counts.entry(v).or_insert(0) +=1;
    }

    // get the highest frequency
    let max_freq = counts.values().copied().max().unwrap();

    let mut modes: Vec<i32> = counts.into_iter().
                          filter_map(|(val, freq)| if freq == max_freq { Some(val) } else {None}).
                          collect();
    modes.sort_unstable();

    Some((median,modes))
}
fn main(){
    let data_1 = vec![3,6,7,8,9,12,9];
    let data_2: Vec<i32>= vec![];

    match median_mode(&data_1) {
        Some((median, modes)) => {
            println!("The median for the give list is {median:?}");
            println!("The value of mode is {modes:?}");
        }
        None => println!("The list is empty")
        
    }

    match median_mode(&data_2) {
        Some((median, modes)) =>{
            println!("The median value is {median:?}");
            println!("The mode value is {modes:?}");
        }
        None => println!("The list is blank!")
        
    }

}