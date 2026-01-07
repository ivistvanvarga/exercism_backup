pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    //todo!("based on the {diagram}, determine the plants the {student} is responsible for");
    let plants = [
        "violets", "radishes", "clover", "grass"
    ];
    let students = [
        "Alice", "Bob", "Charlie", "David", "Eve", "Fred", "Ginny", "Harriet", "Ileana", "Joseph",
        "Kincaid", "Larry",
    ];
    let rows: Vec<&str> = diagram.lines().collect();
    let mut result = Vec::new();
    if let Some(index) = students.iter().position(|&s| s == student) {
        let first_row = rows[0];
        let second_row = rows[1];
        let first_row_plants = &first_row[index * 2..index * 2 + 2];
        let second_row_plants = &second_row[index * 2..index * 2 + 2];
        for ch in first_row_plants.chars().chain(second_row_plants.chars()) {
            match ch {
                'V' => result.push(plants[0]),
                'R' => result.push(plants[1]),
                'C' => result.push(plants[2]),
                'G' => result.push(plants[3]),
                _ => (),
            }
        }
    }
    result
}
