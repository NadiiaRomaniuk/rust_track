pub fn is_armstrong_number(num: u32) -> bool {
    let dig = if num == 0 {
        1
    } else {
        ((num as f64).log10().floor() as u32) + 1
    };

    let mut sum = 0;
    let mut n = num;

    while n > 0 {
        let d = n % 10;
        sum += d.pow(dig);
        n /= 10;
    }

    sum == num
}

