/*
 * @lc app=leetcode.cn id=1488 lang=rust
 *
 * [1488] 避免洪水泛滥
 */



// @lc code=start
use std::collections::{HashMap, HashSet};


impl Solution {
    pub fn avoid_flood(mut rains: Vec<i32>) -> Vec<i32> {
        let mut map = HashMap::<i32, i32>::new();
        let mut cnt = 0;
        let mut res: Vec<i32> = vec![-1; rains.len()];
        
        let mut set = vec![];

        let mut start = 0;
        while start < rains.len() && rains[start] == 0 {
            res[start] = 1;
            start += 1;
        }

        let mut end = rains.len();
        while end > start && rains[end - 1] == 0 {
            end -= 1;
            res[end] = 1;
        }

        for (i, value) in rains.iter().enumerate(){
            let key = i as i32;

            if rains[i] == 0 {
                cnt += 1;
                set.push(key);

                
            } else {
                if map.contains_key(&rains[i]) {
                    if cnt > 0 {
                        cnt -= 1;
                        
                        let rain_index = map.get(&rains[i]).unwrap();

                        let mut set_index = None;

                        for (i, zero_index) in set.iter().enumerate() {
                            if *zero_index > *rain_index {
                                set_index = Some(i);

                                break;
                            }
                        }

                        let mut res_zero = 0;

                        if let Some(index) = set_index {
                            res_zero = set[index];
                            set.remove(index);
                        } else {
                            return vec![];
                        }

                        // let zero_index = set.pop();

                        // let up = zero_index;
                        res[res_zero as usize] = rains[i];     

                        map.insert(rains[i], key);

                    }
                    else {
                        return vec![];
                    } 
                } else {
                    map.insert(rains[i], key);
                }
            }

            // println!("{:?} {:?} {:?}", res, map, set);

        }

        // for (i, value) in map.iter().enumerate() {
        //     if let (k, v) = value {

        //         let mut tmpi = i as i32;



        //         map.remove(&tmpi);


        //     }

        // }

        for (i, value) in set.iter().enumerate() {
            
            res[*value as usize] = 1;
        }

        res
    }
}
// @lc code=end

