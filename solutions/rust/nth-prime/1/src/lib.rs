pub fn nth(n: u32) -> u32 {
    let mut cnt = 0;
    let mut search = 2;


    
    loop {
        if is_prime(search) {
            if cnt == n {
                return search;
            }cnt=cnt+1;
        }search= search+1;
    }}

fn is_prime(num: u32) -> bool {
    if num<2 {
        return false;
    }
    for i in 2..num {
        if num%i == 0 {
            return false;
        }
    }true
}
