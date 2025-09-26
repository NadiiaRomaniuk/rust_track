pub fn factors(mut n: u64) -> Vec<u64> {
    let mut res = Vec::new();
    let mut div = 2;


    
    while n>1 {
        if n%div == 0 {
            res.push(div);
            n = n/div;
        } else {
            div= div+1;
        }
    } res
}
