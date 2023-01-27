#![allow(dead_code)]

#[derive(Clone, Copy, Debug)]
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