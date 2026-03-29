class Solution:

    def encode(self, strs: List[str]) -> str:
        output = ""
        for st in strs:
            output += str(len(st)) + "Ð" + st
        return output

    def decode(self, s: str) -> List[str]:
        output = []
        i = 0
        while i < len(s):
            j = i
            while s[j] != "Ð":
                j += 1
            st_len = int(s[i:j])
            word = s[j+1:j+1+st_len]
            output.append(word)
            i = j + 1 + st_len
        return output

