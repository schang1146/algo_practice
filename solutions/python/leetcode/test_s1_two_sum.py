from .s1_two_sum import Solution


class TestTwoSum:
    def test_example_1(self):
        solution = Solution().twoSum([2, 7, 11, 15], 9)
        solution.sort()
        assert solution == [0, 1]

    def test_example_2(self):
        solution = Solution().twoSum([3, 2, 4], 6)
        solution.sort()
        assert solution == [1, 2]

    def test_example_3(self):
        solution = Solution().twoSum([3, 3], 6)
        solution.sort()
        assert solution == [0, 1]
