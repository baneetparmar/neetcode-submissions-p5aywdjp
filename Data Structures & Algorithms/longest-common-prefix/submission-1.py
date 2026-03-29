class Solution:
    def longestCommonPrefix(self, strs: List[str]) -> str:
        prefix = min(strs,key=len)
        while prefix:
            for s in strs:
                if not s.startswith(prefix):
                    prefix = prefix[:-1]
                    if not prefix:
                        return ""
                    break
            else:
                return prefix
        return ""

