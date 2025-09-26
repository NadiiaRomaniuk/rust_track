pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut res = Vec::new();

    for &fact in factors {
        if fact==0 {
            continue;
        }

        
        let mut mult=fact;
        while mult<limit {
            if !res.contains(&mult) {
                res.push(mult);
            } mult=mult+fact;
        }
    }res.iter().sum()
}
