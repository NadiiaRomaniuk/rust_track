pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack: Vec<char>= Vec::new();

    for ch in string.chars() {
        if ch=='('||ch =='['||ch =='{'{
            stack.push(ch);
        } else {
            if ch==')' {
                if stack.len()==0 { return false; }
                let last=stack.pop().unwrap();
                if last != '(' { return false; }
            } else if ch == ']' {
                if stack.len()==0 { return false; }
                let last=stack.pop().unwrap();
                if last!='[' { return false; }
            } else if ch=='}' {
                if stack.len()== 0 { return false; }
                let last=stack.pop().unwrap();
                if last!='{' { return false; }
            } else {
            }
        }
    }
    if stack.len() == 0 {true
    } else {false
    }
}
