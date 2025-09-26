pub fn series(digits: &str, n: usize) -> Vec<String> {
    let mut res = Vec::new();

    if n==0||n>digits.len(){
        return res;
    }

    for i in 0..=digits.len()-n{
        let cut = &digits[i..i+n]; 
        res.push(cut.to_string());
    }res
}
