pub fn egg_count(display_value: u32) -> usize {
    //todo!("count the eggs in {display_value}")
    let mut count = 0;
    let mut n = display_value;
    while n > 0 {
        count += (n & 1) as usize;
        n >>= 1;
    }
    count
}