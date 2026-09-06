use std::collections::HashMap;
use std::collections::HashSet;

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let (left, result) = input.split_once("==")?;
    let addends: Vec<Vec<char>> = left
        .split('+')
        .map(|word| word.trim().chars().rev().collect())
        .collect();
    let result: Vec<char> = result.trim().chars().rev().collect();

    if addends.iter().any(|word| word.is_empty()) || result.is_empty() {
        return None;
    }

    let leading: HashSet<char> = addends
        .iter()
        .chain(std::iter::once(&result))
        .filter(|word| word.len() > 1)
        .filter_map(|word| word.last().copied())
        .collect();

    let mut letters = HashSet::new();
    for word in &addends {
        letters.extend(word.iter().copied());
    }
    letters.extend(result.iter().copied());
    if letters.len() > 10 {
        return None;
    }

    let mut assignment = HashMap::new();
    solve_columns(
        0,
        0,
        &addends,
        &result,
        &leading,
        &mut assignment,
        0,
    )
}

fn solve_columns(
    column: usize,
    carry: u32,
    addends: &[Vec<char>],
    result: &[char],
    leading: &HashSet<char>,
    assignment: &mut HashMap<char, u8>,
    used: u16,
) -> Option<HashMap<char, u8>> {
    if column == result.len() {
        return (carry == 0 && assignment.len() == all_letters(addends, result).len())
            .then(|| assignment.clone());
    }

    let mut column_letters = Vec::new();
    for word in addends {
        if let Some(&letter) = word.get(column)
            && !column_letters.contains(&letter) && !assignment.contains_key(&letter) {
                column_letters.push(letter);
            }
    }

    assign_column_letters(
        0,
        &column_letters,
        column,
        carry,
        addends,
        result,
        leading,
        assignment,
        used,
    )
}

fn assign_column_letters(
    index: usize,
    column_letters: &[char],
    column: usize,
    carry: u32,
    addends: &[Vec<char>],
    result: &[char],
    leading: &HashSet<char>,
    assignment: &mut HashMap<char, u8>,
    used: u16,
) -> Option<HashMap<char, u8>> {
    if index < column_letters.len() {
        let letter = column_letters[index];
        for digit in 0..=9u8 {
            if used & (1 << digit) != 0 || (digit == 0 && leading.contains(&letter)) {
                continue;
            }
            assignment.insert(letter, digit);
            if let Some(solution) = assign_column_letters(
                index + 1,
                column_letters,
                column,
                carry,
                addends,
                result,
                leading,
                assignment,
                used | (1 << digit),
            ) {
                return Some(solution);
            }
            assignment.remove(&letter);
        }
        return None;
    }

    let sum = addends
        .iter()
        .filter_map(|word| word.get(column))
        .map(|letter| u32::from(assignment[letter]))
        .sum::<u32>()
        + carry;
    let result_digit = (sum % 10) as u8;
    let next_carry = sum / 10;
    let result_letter = result[column];

    if let Some(&assigned_digit) = assignment.get(&result_letter) {
        if assigned_digit != result_digit {
            return None;
        }
        return solve_columns(
            column + 1,
            next_carry,
            addends,
            result,
            leading,
            assignment,
            used,
        );
    }

    if used & (1 << result_digit) != 0
        || (result_digit == 0 && leading.contains(&result_letter))
    {
        return None;
    }

    assignment.insert(result_letter, result_digit);
    let solution = solve_columns(
        column + 1,
        next_carry,
        addends,
        result,
        leading,
        assignment,
        used | (1 << result_digit),
    );
    if solution.is_none() {
        assignment.remove(&result_letter);
    }
    solution
}

fn all_letters(addends: &[Vec<char>], result: &[char]) -> HashSet<char> {
    addends
        .iter()
        .chain(std::iter::once(&result.to_vec()))
        .flat_map(|word| word.iter().copied())
        .collect()
}
