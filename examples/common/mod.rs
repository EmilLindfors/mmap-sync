use bincode::{Decode, Encode};

/// Example data-structure shared between writer and reader(s)
#[derive(Encode, Decode, Debug, PartialEq)]
pub struct HelloWorld {
    pub version: u32,
    pub messages: Vec<String>,
}
