pub fn reply(message: &str) -> &str {
    let temp = String::from(message);
    let mut temp2 = String::from(temp.trim().to_uppercase());  

    if (temp == temp2 && temp.ends_with("?") ){
        return "Calm down, I know what I'm doing!"
    }else if (temp != temp2 && temp.ends_with("?") ){
        return "Sure."
    }
    else if(temp == temp2 ) {
        return "Whoa, chill out!"
    }
    else if (temp2.contains(' ') || temp2.len() == 0)
    {
        return "Fine. Be that way!" 
    }
    
    return "Whatever."
}
