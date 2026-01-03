pub fn raindrops(n: u32) -> String {
    let mut text = String::new();
    if n %3 == 0 { text+="Pling";}
    if n %5 == 0 { text+="Plang";}
    if n %7 == 0 { text+="Plong";}
    if text.len() == 0 { text=n.to_string(); }
    text
}
