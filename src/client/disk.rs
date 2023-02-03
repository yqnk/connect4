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
            Disk::Red => " \x1b[38;5;1mR\x1b[38;5;251m ",
            Disk::Yellow => " \x1b[38;5;220mY\x1b[38;5;251m "
        }
    }
}

// for debug purpose only
impl std::fmt::Display for Disk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}