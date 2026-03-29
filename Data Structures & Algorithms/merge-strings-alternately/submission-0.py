class Solution:
    def mergeAlternately(self, word1: str, word2: str) -> str:
        l = r = 0
        output = []
        while l < len(word1) and r < len(word2):
            output.append(word1[l] + word2[r])
            l += 1
            r += 1
        if len(word1) > len(word2):
            output.extend(word1[l:])
        else:
            output.extend(word2[r:])
        return "".join(output)
        