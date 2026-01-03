pub fn build_proverb(list: &[&str]) -> String {
    
    let mut text = Vec::new();
    
    if list.len() > 0 {
        for  i in 0..list.len()-1{
            text.push(format!("For want of a {} the {} was lost.", list[i], list[i + 1]));
        }
        text.push(format!("And all for the want of a {}.",list[0]));
    }
    text.join("\n")
}
