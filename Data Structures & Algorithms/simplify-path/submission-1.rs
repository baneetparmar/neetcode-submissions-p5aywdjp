impl Solution {
    pub fn simplify_path(path: String) -> String {
        let mut stack: Vec<&str> = vec![];

        for part in path.split('/') {
            match part {
                "" | "." => {},           // skip empty and current dir
                ".." => { stack.pop(); }, // go up
                dir => stack.push(dir),   // valid directory name
            }
        }

        format!("/{}", stack.join("/"))
    }
}