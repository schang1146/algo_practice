class Solution:
    def solve(self, multiples, max):
        running_sum = 0

        for num in range(1, max):
            to_add = False
            for multiple in multiples:
                if num % multiple == 0:
                    to_add = True
                    break

            if to_add:
                print(f"adding {num}")
                running_sum += num

        return running_sum


if __name__ == "__main__":
    solution = Solution().solve([3, 5], 1000)
    print(f"Solution: {solution}")
