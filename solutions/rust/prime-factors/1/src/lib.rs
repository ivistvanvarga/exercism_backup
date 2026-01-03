pub fn factors(n: u64) -> Vec<u64> {
    let mut primes = Vec::new();
    primes.push(2);
    let mut factores = Vec::new();
    let mut number = 3;

    while number <= ((n as f64).sqrt() as u64)+1 {

        if !primes.iter().any( |x| number % x == 0) {
            primes.push(number); 
        }
        number+=1;
    }
    number = n;

    for p in primes {
        while number % p == 0 {
            factores.push(p);
            number = number/p;
        }
    }
    if factores.len() <1 && n >1 {factores.push(n)}


    return factores;

}
