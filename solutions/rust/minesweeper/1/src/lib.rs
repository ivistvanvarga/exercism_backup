pub fn annotate(minefield: &[&str]) -> Vec<String> {
    //todo!("\nAnnotate each square of the given minefield with the number of mines that surround said square (blank if there are no surrounding mines):\n{minefield:#?}\n");
    let mut field:Vec<Vec<String>> = Vec::new();
    for line in minefield.into_iter(){
        field.push(Vec::from_iter(line.chars().into_iter()));
    }
    let mut result:Vec<String> = Vec::new();
    for line in field{
        result.push(line.concat());
    }
    return result;
}
