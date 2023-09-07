/*
 * @lc app=leetcode id=4 lang=rust
 *
 * [4] Median of Two Sorted Arrays
 */

pub struct Solution {}

// @lc code=start
impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        if nums1.len() > nums2.len() {
            return Solution::find_median_sorted_arrays(nums2, nums1);
        }
        let [m, n] = [nums1.len(), nums2.len()];

        let [mut left, mut right] = [0, m];

        while left <= right {
            let partition_a = (left + right) / 2;
            let partition_b = (m + n + 1) / 2 - partition_a;

            let max_left_a  = if partition_a == 0 { i32::MIN } else { nums1[partition_a - 1] };
            let min_right_a =  if partition_a == m { i32::MAX } else { nums1[partition_a] };

            let max_left_b  = if partition_b == 0 { i32::MIN } else { nums1[partition_b - 1] };
            let min_right_b =  if partition_b == m { i32::MAX } else { nums1[partition_b] };
            
            // A partitioned precisely
            if max_left_a <= min_right_b && min_right_a >= max_left_b {
                return if (m + n) % 2 == 0 {
                    (max_left_a.max(max_left_b) + min_right_a.min(min_right_b)) as f64
                } else {
                    max_left_a.max(max_left_b) as f64
                };
            }
            if max_left_a > min_right_b {
                right = partition_a - 1;
            } else {
                left = partition_a + 1;
            }
        }
        // should only happen on empty arrays
        return 0f64
    }
}
// @lc code=end
