pub fn build_proverb(list: &[&str]) -> String {
    /*let mut text :Vec<_> = list.into_iter().map(|x| {
        "For want of a {0} the {1} was lost.".replace("{0}", x)
    });*/
    let mut text = Vec::new();
    /*if list.len() > 0 {
        /*while text.len() <= list.len() -1 {
            let mut word = list.to_vec().into_iter().cycle();
            text.push("For want of a {0} the {1} was lost.".replace("{0}", &word.next().unwrap().to_string()).replace("{1}", &word.next().unwrap().to_string()));
        }*/
        text.push("And all for the want of a {0}.".replace("{0}",&list.first().unwrap().to_string()));
    }*/
    if list.len() > 0 {
        if list.len() >1 {
            let mut word = list.to_vec().into_iter().cycle();
            for  _ in 1..list.len(){
                text.push("For want of a {0} the {1} was lost.".replace("{0}", &word.next().unwrap().to_string()).replace("{1}", &word.next().unwrap().to_string()));
            }
        }
        text.push("And all for the want of a {0}.".replace("{0}",&list.first().unwrap().to_string()));
    }
    text.join("\n")
}
