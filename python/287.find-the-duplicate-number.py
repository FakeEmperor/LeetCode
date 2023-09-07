#
# @lc app=leetcode id=287 lang=python3
#
# [287] Find the Duplicate Number
#
from typing import List


# @lc code=start
class Solution:
    def findDuplicate(self, nums: List[int]) -> int:
        last_index = -1
        i = 0
        while last_index != i:
            last_index = i
            i = nums[i]
        return i

        
# @lc code=end


print(Solution().findDuplicate([5,3,4,5,1,2]))