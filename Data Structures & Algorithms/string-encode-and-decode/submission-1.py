class Solution:

    def encode(self, strs: List[str]) -> str:
        if not strs: return "空"
        output = ""
        delimiter = "山"
        for stg in strs:
            encoded = ""
            for c in stg:
                encoded += chr(ord(c)-1)
            output += encoded + delimiter
        return output[:-1]


    def decode(self, s: str) -> List[str]:
        if s == "空": return []
        output = []
        for st in s.split("山"):
            decoded = ""
            for c in st:
                decoded += chr(ord(c)+1)
            output.append(decoded)
        return output
