use std::vec;

pub fn verse(n: u32) -> String {
    let mut verse=vec![];
   
    match n {
        0 => {verse.push("No more bottles of beer on the wall, no more bottles of beer."); verse.push("Go to the store and buy some more, 99 bottles of beer on the wall.")},
        1 => {verse.push("1 bottle of beer on the wall, 1 bottle of beer."); verse.push("Take it down and pass it around, no more bottles of beer on the wall.")},
        2 => {verse.push("{0} bottles of beer on the wall, {0} bottles of beer."); verse.push("Take one down and pass it around, {1} bottle of beer on the wall.")},
        _ => {verse.push("{0} bottles of beer on the wall, {0} bottles of beer."); verse.push("Take one down and pass it around, {1} bottles of beer on the wall.")}
    }
    
    return (verse.join("\n")+"\n").replace("{0}", n.to_string().as_str()).replace("{1}", ({if n>0 { n-1} else {n}}).to_string().as_str())
}

pub fn sing(start: u32, end: u32) -> String {
    (end..=start).rev().map(|x| verse(x)).collect::<Vec<String>>().join("\n")
}
