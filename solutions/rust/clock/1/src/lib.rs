//use std::fmt;
#[derive(Debug, Clone, Copy)]
pub struct Clock{
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self { Self {
        hours:  ((hours*60+minutes)/60)  % 24,
        minutes: minutes % 60,
    }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self { Self {
        minutes : (self.minutes+minutes) % 60,
        hours : (self.hours*60+self.minutes+minutes)/60 % 24
        }
    }
    
    pub fn to_string(&self) -> String {
       let mut tmp = String::from("");
        if self.hours<10 { tmp.push('0');}
        tmp.push_str(&self.hours.to_string());
        tmp.push(':');
        if self.minutes<10 {tmp.push('0');}
        tmp.push_str(&self.minutes.to_string());
        return tmp;

    }
    
}
/*
impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.hours self.minutes)
    }
}*/

impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        self.hours == other.hours && self.minutes == other.minutes
    }
}
