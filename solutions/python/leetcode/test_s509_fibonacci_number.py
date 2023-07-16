from .s509_fibonacci_number import Solution


class TestFib:
    def test_example_1(self):
        solution = Solution().fib(2)
        assert solution == 1

    def test_example_2(self):
        solution = Solution().fib(3)
        assert solution == 2

    def test_example_3(self):
        solution = Solution().fib(4)
        assert solution == 3

    def test_smallest_input(self):
        solution = Solution().fib(0)
        assert solution == 0

    def test_largest_input(self):
        solution = Solution().fib(30)
        assert solution == 832040
