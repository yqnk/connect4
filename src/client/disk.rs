#![allow(dead_code)]

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Disk {
    Red, 
    Yellow,
    None
}

impl Disk {
    pub fn as_str(&self) -> &str {
        match self {
            Disk::None => "   ",
            Disk::Red => " R ",
            Disk::Yellow => " Y "
        }
    }
}

// for debug purpose only
impl std::fmt::Display for Disk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}