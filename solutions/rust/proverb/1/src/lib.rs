pub fn build_proverb(list: &[&str]) -> String {
    if list.len()==0 {
        return "".to_string();
    }

    let mut res=String::new();

    for i in 0..list.len()-1 {
        res.push_str("For want of a ");
        res.push_str(list[i]);
        res.push_str(" the ");
        res.push_str(list[i + 1]);
        res.push_str(" was lost.\n");
    }
    res.push_str("And all for the want of a ");
    res.push_str(list[0]);
    res.push('.');
    res
}
