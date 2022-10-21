from time import perf_counter


class Solution:
    def fib(self, n: int) -> int:
        fib_data = [0, 1, 1]
        if n < 0:
            raise Exception("Error: Can't get negative Fibonacci numbers.")
        if n <= 2:
            return fib_data[n]
        for _ in range(n-2):
            fib_data = fib_data[1:] + [fib_data[1] + fib_data[2]]
        return fib_data[-1]

if __name__ == "__main__":
    tic = perf_counter()
    solution = Solution().fib(2)
    toc = perf_counter()
    print(f"Solution: {solution}")
    print(f"Time elapsed: {toc - tic} sec(s)")