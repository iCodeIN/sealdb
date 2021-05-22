use sealdb::{Record, types::FixedLenStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black,
    Blue,
    Cyan,
    Green,
    Magenta,
    Red,
    White,
    Yellow,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LabelId(usize);

#[derive(Debug, Clone, PartialEq)]
pub struct Label {
    pub id: LabelId,
    pub name: FixedLenStr<256>,
    pub color: Option<Color>,
}

impl Record for Label {}
