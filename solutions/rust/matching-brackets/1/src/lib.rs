pub fn brackets_are_balanced(string: &str) -> bool {
    //todo!("Check if the string \"{string}\" contains balanced brackets");
    let mut stack: Vec<char> = Vec::new();
    for c in string.chars() {
        match c {
            '(' | '{' | '[' => stack.push(c),
            ')' => {
                if stack.pop() != Some('(') {
                    return false;
                }
            }
            '}' => {
                if stack.pop() != Some('{') {
                    return false;
                }
            }
            ']' => {
                if stack.pop() != Some('[') {
                    return false;
                }
            }
            _ => {}
        }
    }
    stack.is_empty()

}
