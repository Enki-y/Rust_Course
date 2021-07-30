
extern crate rand;

use rand::Rng;

fn main() {
    let array_nums = [1u32,2u32,3u32];
    println!("sum of following array {:?}", &array_nums);
    println!("result: {:?}", sum(&array_nums))
}


fn sum(nums: &[u32]) -> Option<u32>{
    let mut result = 0u32;
    for i in nums.iter(){
        match result.checked_add(*i){
            Some(x) => result = x,
            None => return None
        }
    }
    return Some(result);
}
