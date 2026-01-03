use std::collections::HashMap;
use std::thread;
use std::sync::{Arc, Mutex};


pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let result = Arc::new(Mutex::new(HashMap::new()));
    let input_hm: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::from_iter(input.iter().map(|s| String::from_iter(s.to_lowercase().chars().filter(|c| c.is_alphabetic()))))));
    
    

    let mut handles = vec![];
    for _ in 0..worker_count {
        let input_v= Arc::clone(&input_hm);
        let result_v = Arc::clone(&result);
        let handle = thread::spawn(move || {
            loop {
                let line= input_v.lock().unwrap().pop();            
                if !line.is_none() {
                    for chr in line.unwrap().chars() {
                        result_v.lock().unwrap().entry(chr).and_modify(|counter| *counter += 1).or_insert(1);                  
                    }
                }else {
                    break;
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
