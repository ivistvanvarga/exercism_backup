use std::u32;


/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    //todo!("Is the Luhn checksum for {code} valid?");
    
    if  code.chars().filter(|c| c.is_digit(10)).take(2).count() > 1 && code.chars().all(|c| c.is_digit(10) || c == ' ' ) {
          
      let sum : u32=code.chars().filter_map(|c| c.to_digit(10)).rev().enumerate().map(|(index,number)| if index %2 == 0 {number} else { if(number*2)>9 {(number*2)-9} else {number*2}}).sum::<u32>();
  
      return sum % 10 == 0;   
    
  }
    return false;
}
