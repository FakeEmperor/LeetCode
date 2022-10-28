/*
 * @lc app=leetcode id=88 lang=rust
 *
 * [88] Merge Sorted Array
 */

pub struct Solution {}

// @lc code=start
impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut nums1_copy = Vec::<i32>::with_capacity(m as usize);
        nums1_copy.extend(nums1.iter().take(m as usize));
        let mut nums1_counter = 0usize;
        let mut nums2_counter = 0usize;
        for i in 0usize..m as usize + n as usize {
            if nums1_counter >= m as usize {
                nums1.splice(i.., nums2.iter().skip(nums2_counter).map(|x| *x));
                return;
            } else if nums2_counter >= n as usize {
                nums1.splice(i.., nums1_copy.iter().skip(nums1_counter).map(|x| *x));
                return;
            }
            let n1 = nums1_copy[nums1_counter];
            let n2 = nums2[nums2_counter];
            if n1 > n2 {
                nums2_counter += 1;
            } else {
                nums1_counter += 1;
            }
            nums1[i] = n1.min(n2);
        }
    }
}
// @lc code=end
