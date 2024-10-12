// fn main() {
//     println!("Hello, world!");
// }

// use std::collections::HashMap;

// impl Solution {
//     pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
//         let mut map = HashMap::new();
//         for (i, &value) in nums.iter().enumerate() {
//             let complement = target - value;
//             if map.contains_key(&complement) {
//                 return vec![*map.get(&complement).unwrap() as i32, i as i32];
//             }
//             map.insert(value, i);
//         }
//         vec![]
//     }
// }


use std::env;

fn main() {
    // Try running with a different env var - `PORT=4000 cargo run`
    let port = env::var("PORT").unwrap_or("3000".to_string());
    println!("{}", port);
}