
pub fn annotate(minefield: &[&str]) -> Vec<String> {
    //todo!("\nAnnotate each square of the given minefield with the number of mines that surround said square (blank if there are no surrounding mines):\n{minefield:#?}\n");
  
    let mut result:Vec<String> = Vec::new();
    for (i,line) in minefield.into_iter().enumerate(){
        let mut new_line = String::new();
        for (j,c) in line.chars().enumerate() {
            if c == '*' {
                new_line.push('*');
            } else {
                let mut count = 0;
                for x in i.saturating_sub(1)..=i+1 {
                    for y in j.saturating_sub(1)..=j+1 {
                        if x<minefield.len() && y < line.len() && minefield[x].chars().nth(y) == Some('*') {
                            count +=1;
                        }
                    }
                }
                if count > 0 {
                    new_line.push_str(&count.to_string()) ;
                }else {
                    new_line.push(' ')
                }
            }
        }
        result.push(new_line);
    }
    
   
    return result;
}
