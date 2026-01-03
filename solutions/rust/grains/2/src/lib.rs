pub fn square(s: u32) -> u64 {
    
    let mut sum:u64 =2;
    if s <1 || s >64 { panic!("Square must be between 1 and 64");}
    sum=sum.pow(s-1);
   sum
}

pub fn total() -> u64 {
    (1..65).map(square).sum()
}
