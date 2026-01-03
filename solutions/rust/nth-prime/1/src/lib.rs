pub fn nth(n: u32) -> u32 {

  let mut primes = Vec::new();
  primes.push(2);
  primes.push(3);
  let mut number = 4;
  let limit :usize = n.try_into().unwrap();
  while primes.len() <= limit {
    let mut is_prime = true;
    
    for p in &primes {
        if number % p ==0 {
            is_prime =false;
            break;
        } 
    }
    if is_prime {
       primes.push(number); 
    }
    number+=1;
  }  

  return *primes.get(limit).unwrap();   
}


