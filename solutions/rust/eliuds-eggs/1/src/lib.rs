pub fn egg_count(display_value: u32) -> usize {
    let mut cnt = 0;
    let mut n = display_value;

    while n>0 {
        if n%2==1 {
            cnt=cnt+1;
        }n=n/2;
    }cnt
}
