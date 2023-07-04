from typing import List


class Solution:
    def twoSum(self, nums: List[int], target: int) -> List[int]:
        diff_dict = {}

        for idx in range(len(nums)):
            if target - nums[idx] in diff_dict:
                return [idx, diff_dict[target - nums[idx]]]
            diff_dict[nums[idx]] = idx


if __name__ == "__main__":
    solution = Solution().twoSum([2, 7, 11, 15], 9)
    print(f"Solution: {solution}")
