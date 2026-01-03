pub fn annotate(garden: &[&str]) -> Vec<String> {
    let mut verse: Vec<String> = vec![];
    for (i, row) in garden.iter().enumerate() {
        let mut annotated_row = String::new();
        for (j, &cell) in row.as_bytes().iter().enumerate() {
            if cell == b' ' {
                let mut count = 0;
                for di in -1..=1 {
                    for dj in -1..=1 {
                        if di == 0 && dj == 0 {
                            continue;
                        }
                        let ni = i as isize + di;
                        let nj = j as isize + dj;
                        if ni >= 0 && ni < garden.len() as isize {
                            let neighbor_row = garden[ni as usize].as_bytes();
                            if nj >= 0 && nj < neighbor_row.len() as isize {
                                if neighbor_row[nj as usize] == b'*' {
                                    count += 1;
                                }
                            }
                        }
                    }
                }
                if count > 0 {
                    annotated_row.push((b'0' + count) as char);
                } else {
                    annotated_row.push(' ');
                }
            } else {
                annotated_row.push('*');
            }
        }
        verse.push(annotated_row);
    }
    verse
}