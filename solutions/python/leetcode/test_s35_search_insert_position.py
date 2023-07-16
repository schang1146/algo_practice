from .s35_search_insert_position import Solution


class TestSearchInsert:
    def test_example_1(self):
        solution = Solution().searchInsert([1, 3, 5, 6], 5)
        assert solution == 2

    def test_example_2(self):
        solution = Solution().searchInsert([1, 3, 5, 6], 2)
        assert solution == 1

    def test_example_3(self):
        solution = Solution().searchInsert([1, 3, 5, 6], 7)
        assert solution == 4

    def test_exists_first(self):
        solution = Solution().searchInsert([1, 3, 5, 6], 1)
        assert solution == 0

    def test_exists_last(self):
        solution = Solution().searchInsert([1, 3, 5, 6], 6)
        assert solution == 3

    def test_short_input_exists_1(self):
        solution = Solution().searchInsert([5], 5)
        assert solution == 0

    def test_short_input_exists_2(self):
        solution = Solution().searchInsert([3, 5], 5)
        assert solution == 1

    def test_short_input_missing_less_1(self):
        solution = Solution().searchInsert([5], 2)
        assert solution == 0

    def test_short_input_missing_less_2(self):
        solution = Solution().searchInsert([3, 5], 2)
        assert solution == 0

    def test_short_input_missing_more_1(self):
        solution = Solution().searchInsert([5], 9)
        assert solution == 1

    def test_short_input_missing_more_1(self):
        solution = Solution().searchInsert([3, 5], 9)
        assert solution == 2
