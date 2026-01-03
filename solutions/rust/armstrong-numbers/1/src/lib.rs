
pub fn is_armstrong_number(num: u32) -> bool {
    
    let num_s = num.to_string();
    let result = num_s.chars().map(|c|  c.to_digit(10).unwrap()).rev().enumerate().map(|(_index,number)| number.pow((1*num_s.len()).try_into().unwrap()) ).sum::<u32>();

    return result == num ;

}
