pub fn square(s: u32) -> u64 {
    if s == 0 || s > 64 {
        panic!("квадрт мусить бути між 1 та 64");
    }
    let mut grains: u64 = 1;
    for _ in 1..s {
        grains=grains*2;
    }grains
}

pub fn total() -> u64 {
    let mut suma: u64 = 0;
    for i in 1..=64 {
        suma= suma+square(i);
    }suma
}
