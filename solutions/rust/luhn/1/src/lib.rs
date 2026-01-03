
/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    //todo!("Is the Luhn checksum for {code} valid?");
    let mut code_str = String::from(code.trim());
    code_str.retain(|c| c != ' ');
    if  code_str.len() > 1 {
      
      if code_str.parse::<u32>().is_err() {
        return false;
      }
      let mut sum=0;
      for (i,c) in code_str.chars().enumerate() {
        if !c.is_digit(10){
          return false;
        }
        let mut n :u32 = c.to_string().parse().unwrap() ;
        
        if i %2 == 0 {          
          if n*2 >9 {
            n = n*2-9; 
          }else {
              n*=2;
          }
          n = (n*2 )%9;
          sum+=n;  
      }      
      print!(">>{}",sum);
      return sum % 10 == 0;   
    }
  }
    return false;
}
