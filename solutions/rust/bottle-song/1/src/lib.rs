use std::vec;
pub fn recite(start_bottles: u32, take_down: u32) -> String {
   return  (take_down..=start_bottles).rev().map(|x| verse(x)).collect::<Vec<String>>().join("\n")
}

pub fn verse(n: u32) -> String {
    let mut verse=vec![];
    let mut x=String::new();
    let mut y=String::new();

    match n {
        0 => {x=" ".to_string(); y=" ".to_string()},
        1 => {x="One".to_string(); y="no".to_string()},
        2 => {x="Two".to_string(); y="one".to_string()},
        3 => {x="Three".to_string(); y="two".to_string()},
        3 => {x="Three".to_string(); y="two".to_string()},
        4 => {x="Four".to_string(); y="three".to_string()},
        5 => {x="Five".to_string(); y="four".to_string()},
        6 => {x="Six".to_string(); y="five".to_string()},
        7 => {x="Seven".to_string(); y="six".to_string()},
        8 => {x="Eight".to_string(); y="seven".to_string()},
        9 => {x="Nine".to_string(); y="eight".to_string()},
        10 => {x="Ten".to_string(); y="nine".to_string()},
        _ => {x=format!("{} bottles", n); y=format!("{} bottles", n-1)}
    }

    verse.push("{0} green bottles hanging on the wall,");
    verse.push("{0} green bottles hanging on the wall,");
    verse.push("And if one green bottle should accidentally fall,");
    verse.push("There'll be {1} green bottles hanging on the wall.");

    return (verse.join("\n")+"\n").replace("{0}",&x).replace("{1}", &y)
}
