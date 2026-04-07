/*
 * @lc app=leetcode.cn id=1385 lang=rust
 *
 * [1385] 两个数组间的距离值
 */

// @lc code=start
impl Solution {
    pub fn find_the_distance_value(arr1: Vec<i32>, arr2: Vec<i32>, d: i32) -> i32 {


        let mut tmparr2 = arr2;

        tmparr2.sort_unstable_by(|a, b| a.cmp(b));

        println!("{:?}", tmparr2);

        let mut ans = 0;

        let mut tmparr1 = arr1.clone();

        tmparr1.iter().for_each(|item| {
            let mut flag = false;
            let mut left = 0;
            let mut right = tmparr2.len();

            while left < right {
                let mid = left + (right - left) / 2;
    
                if  tmparr2[mid] <= d + *item && tmparr2[mid] >= *item - d  {
                    
                    flag = true;
                    break; 
                } else if tmparr2[mid] > d + *item {
                    right = mid
                } else if tmparr2[mid] <  *item - d {
                    left = mid + 1
                }
                
            }
            if !flag {ans += 1};

        });

        ans

    }
}
// @lc code=end

