use std::fmt;
#[derive(Debug, Clone, Copy,Eq, PartialEq)]
pub struct Clock{
    
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self { Self {
        
        minutes: (((hours*60+minutes)%(24*60))+24*60) % (24*60),
    }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self { 
        Self::new(0,self.minutes+minutes)
        
    }    
       
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.minutes/60,self.minutes % 60)
    }
}


