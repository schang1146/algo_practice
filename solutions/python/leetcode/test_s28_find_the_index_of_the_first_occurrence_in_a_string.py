from .s28_find_the_index_of_the_first_occurrence_in_a_string import Solution


class TestFindTheIndexOfTheFirstOccurrenceInAString:
    def test_example_1(self):
        solution = Solution().strStr("sadbutsad", "sad")
        assert solution == 0

    def test_example_2(self):
        solution = Solution().strStr("leetcode", "leeto")
        assert solution == -1

    def test_haystack_shorter_than_needle_1(self):
        solution = Solution().strStr("fjioew", "fewioajfeiowa")
        assert solution == -1

    def test_haystack_shorter_than_needle_2(self):
        solution = Solution().strStr("aaa", "aaaa")
        assert solution == -1

    def test_haystack_equals_needle(self):
        solution = Solution().strStr("jfeiowajfieowajf", "jfeiowajfieowajf")
        assert solution == 0

    def test_shortest_haystack_equals_needle(self):
        solution = Solution().strStr("a", "a")
        assert solution == 0

    def test_shortest_haystack_does_not_equal_needle(self):
        solution = Solution().strStr("a", "b")
        assert solution == -1

    def test_needle_at_end_of_haystack(self):
        solution = Solution().strStr("abcabcxyz", "xyz")
        assert solution == 6

    def test_multiple_valid_needles_found(self):
        solution = Solution().strStr("abcdefabcdefabcdef", "cde")
        assert solution == 2

    def test_overlapping_needles(self):
        solution = Solution().strStr("mississippi", "issipi")
        assert solution == -1
