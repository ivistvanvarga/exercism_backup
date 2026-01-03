pub fn reply(message: &str) -> &str {
   

    if message.trim().is_empty() { return "Fine. Be that way!" }
    let yell = message.chars().any(char::is_alphabetic) && message.to_uppercase() == message;
    let question = message.trim().ends_with('?');    
    
    match (yell,question){
        (true, false) => "Whoa, chill out!",
        (false, true) => "Sure.",
        (true, true) => "Calm down, I know what I'm doing!",
        _ => "Whatever."
    }
}
