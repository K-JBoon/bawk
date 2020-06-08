use std::fmt;

#[derive(Debug, Clone)]
pub enum BawkType {
    Text(String),
    Integer(i64),
    None
}

impl fmt::Display for BawkType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        
        match self {
            BawkType::Text(v) => write!(f, "{}", v),
            BawkType::Integer(v) => write!(f, "{}", v),
            _ => write!(f, "") 
        }
    }
}
