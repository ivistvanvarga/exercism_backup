pub fn factors(n: u64) -> Vec<u64> {

    let mut factores = Vec::new();
    let mut number = 2;
    let mut _n = n;
    while _n> 1 {
        while _n % number == 0 {
            factores.push(number);
            _n = _n/number;
        }
        number+=1;
    }

    return factores;

}
