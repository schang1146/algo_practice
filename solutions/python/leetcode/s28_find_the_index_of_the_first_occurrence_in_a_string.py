class Solution:
    def strStr(self, haystack: str, needle: str) -> int:
        if len(haystack) < len(needle):
            return -1

        for idx in range(len(haystack) - len(needle) + 1):
            needle_found = True
            for sub_idx in range(len(needle)):
                if haystack[idx + sub_idx] != needle[sub_idx]:
                    needle_found = False
                    break
            if needle_found:
                return idx
        return -1


if __name__ == "__main__":
    solution = Solution().strStr("sadbutsad", "sad")
    print(f"Solution: {solution}")
