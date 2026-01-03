pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut tmp: Vec<Vec<u32>> = Vec::new();
    for f in factors {
        let mut mult=  Vec::new();
        for i in 1..limit{
            let t = i*f;
            if t <limit {
                mult.push(t);
            }else {
                break;
            }
        }
        tmp.push(mult);
    }
    let mut sum=tmp.concat();
    sum.sort();
    sum.dedup();
    return sum.into_iter().sum();
}
