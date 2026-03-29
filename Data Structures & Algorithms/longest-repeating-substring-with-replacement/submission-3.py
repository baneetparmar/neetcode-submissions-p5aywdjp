class Solution:
    def characterReplacement(self, s: str, k: int) -> int:
        l = 0
        left = right = 0 # pointers at each end of sliding window
        mp = defaultdict(int) # element in the sliding window

        while right < len(s):
            mp[s[right]] += 1
            maxF = max(mp.values())
            wlen = right - left + 1 # left/right are index and not actual distance


            # window length - maximum frequency = characters to be replaced
            if wlen - maxF <= k: # window validity check
                l = max(l, right - left + 1)
            else: # window is invalid
                mp[s[left]] -= 1 # try and remove a char to make window valid again
                left += 1 # shrink window
            right += 1 # move window

        return l
            