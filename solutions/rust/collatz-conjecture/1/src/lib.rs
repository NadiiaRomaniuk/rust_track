pub fn collatz(n: u64) -> Option<u64> {
    if n==0 {
        return None;
    }

    let mut cnt=0;
    let mut curr=n;

    while curr != 1 {
        if curr%2==0 {
            curr=curr/2;
        } else {
            curr=curr*3+1;
        }cnt=cnt+1;
    } Some(cnt)
}
