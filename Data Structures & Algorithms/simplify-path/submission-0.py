class Solution:
    def simplifyPath(self, path: str) -> str:
        path = path.split("/")
        print(path)
        res = []
        for s in path:
            match s:
                case "" | ".":
                    pass
                case "..":
                    if len(res) > 0:
                        res.pop()
                case _:
                    res.append(s)
        return "/" + "/".join(res)
            
