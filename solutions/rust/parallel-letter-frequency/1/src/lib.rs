use std::collections::HashMap;
use std::thread;
use std::sync::{Arc, Mutex};


pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let result = Arc::new(Mutex::new(HashMap::new()));
    let input_hm: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
    for line in input {
        input_hm.lock().unwrap().push(line.to_string());
    }

    let mut handles = vec![];
    for _ in 0..worker_count {
        let input_v= Arc::clone(&input_hm);
        let result_v = Arc::clone(&result);
        let handle = thread::spawn(move || {
            let line= input_v.lock().unwrap().pop();
            if !line.is_none() {
                for chr in line.unwrap().chars().filter(|c| c.is_alphabetic()) {
                    if let Some(c) = chr.to_lowercase().next() {
                        (*result_v.lock().unwrap().entry(c).or_insert(0)) += 1;
                    }
                }                
            }
           
        });

        handles.push(handle);
    } 
    for handle in handles {
        handle.join().unwrap();
    }

    return result.lock().unwrap().clone();
}
