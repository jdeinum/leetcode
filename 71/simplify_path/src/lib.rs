pub fn simplify_path(path: String) -> String {
    // get rid of extra / by splitting on / , filtering empty values, and joining them
    let p: Vec<&str> = path.split("/").filter(|x| *x != "").collect();
    let mut final_string: Vec<&str> = Vec::new();
    for x in p {
        match x {
            ".." => {
                let _ = final_string.pop();
            }
            "." => {}
            a => final_string.push(a),
        }
    }
    let s = final_string.join("/");
    let s = format!("/{}", s);
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let x = "/home//foo/".to_string();
        assert_eq!(simplify_path(x), "/home/foo".to_string());
    }

    #[test]
    fn case2() {
        let x = "/home/user/Documents/../Pictures".to_string();
        assert_eq!(simplify_path(x), "/home/user/Pictures".to_string());
    }
}
