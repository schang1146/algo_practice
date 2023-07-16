from typing import List


class Solution:
    def searchInsert(self, nums: List[int], target: int) -> int:
        if target <= nums[0]:
            return 0
        if target == nums[len(nums) - 1]:
            return len(nums) - 1
        if target > nums[len(nums) - 1]:
            return len(nums)

        start = 0
        end = len(nums) - 1
        mid = (end + start) // 2

        while start != mid:
            if nums[mid] == target:
                return mid
            if nums[mid] > target:
                end = mid
                mid = (end + start) // 2
            if nums[mid] < target:
                start = mid
                mid = (end + start) // 2

        return end


if __name__ == "__main__":
    solution = Solution().searchInsert([1, 3, 5, 6], 5)
    print(f"Solution: {solution}")
