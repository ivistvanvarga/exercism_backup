use std::vec;

pub fn verse(n: u32) -> String {
    let mut verse=vec![];
   
    match n {
        0 => verse.push("No more bottles of beer on the wall, no more bottles of beer."),
        1 => verse.push("# bottle of beer on the wall, # bottle of beer."),
        _ => verse.push( "# bottles of beer on the wall, # bottles of beer.")
    }
    match n {
        0 => verse.push("Go to the store and buy some more, 99 bottles of beer on the wall."),
        1 => verse.push("Take it down and pass it around, no more bottles of beer on the wall."),
        2 => verse.push("Take one down and pass it around, ? bottle of beer on the wall."),
        _ => verse.push("Take one down and pass it around, ? bottles of beer on the wall.")
    }  
    return (verse.join("\n")+"\n").replace("#", n.to_string().as_str()).replace("?", ({if n>0 { n-1} else {n}}).to_string().as_str())
}

pub fn sing(start: u32, end: u32) -> String {
    let mut i= start;
    let mut text = vec![];
    while i >= end {
      text.push(verse(i));
      if i == 0 { break;        
      }
      i-=1;  
    } 
    return text.join("\n") ;
}
