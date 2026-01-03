pub fn is_armstrong_number(num: u32) -> bool {
    
    if num <10 {return true;} else {
        let num = u64::from(num);
        let result = num.to_string().chars().map(|c| (c.to_digit(10).unwrap() as u64).pow(num.ilog10()+1)).sum::<u64>();
        return  num == result;
    }

}
