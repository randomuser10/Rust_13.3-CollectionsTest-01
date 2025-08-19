// to bring HashMaps in current scope

use std::collections::HashMap;

//function to check median and mode of a list

fn median_mode(nums: &[i32]) -> Option<f64>{
    // check if list is empty
    if nums.is_empty(){
        return None;
    }
    let mut sorted = nums.to_vec();
    //sort the list
    sorted.sort_unstable();
    //save the length of the list

    let n = sorted.len();

    let median = if n % 2 == 1  {
        sorted[n / 2] as f64       
    } else {
        let a = sorted[n / 2 - 1] as f64;
        let b = sorted[n / 2] as f64;
        (a + b) / 2.0
    };
    Some(median)


}

fn main(){
    let data1 = vec![1,2,3,4,5];

    match median_mode(&data1){
        Some(median) => {
            println!("The median value is {median}");
        }
        None => print!("The list is empty!")
    }

}