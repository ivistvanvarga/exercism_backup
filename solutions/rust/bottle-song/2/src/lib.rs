use std::vec;
pub fn recite(start_bottles: u32, take_down: u32) -> String {
    let mut song = String::new();
    for  i in 0..take_down {
        let n = start_bottles - i;
        let next_n = start_bottles - i - 1;
        let verse = format!(
            "{0} green {1} hanging on the wall,\n\
             {0} green {1} hanging on the wall,\n\
             And if one green bottle should accidentally fall,\n\
             There'll be {2} green {3} hanging on the wall.\n",
            num_to_word(n),
            bottle_word(n),
            num_to_word(next_n).to_lowercase(),
            bottle_word(next_n).to_lowercase()
        );
        song.push_str(&verse);
        if i < take_down - 1 {
            song.push('\n');
        }
    }

   return  song
}

fn num_to_word(n: u32) -> String {
    match n {
        0 => "No".to_string(),
        1 => "One".to_string(),
        2 => "Two".to_string(),
        3 => "Three".to_string(),
        4 => "Four".to_string(),
        5 => "Five".to_string(),
        6 => "Six".to_string(),
        7 => "Seven".to_string(),
        8 => "Eight".to_string(),
        9 => "Nine".to_string(),
        10 => "Ten".to_string(),
        _ => n.to_string()
    }
}

fn bottle_word(n: u32) -> String {
    match n {
        1 => "bottle".to_string(),
        _ => "bottles".to_string()
    }
}
/*
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
}*/
