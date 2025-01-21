use std::fmt::Display;

pub enum Number {
    _2,
    _3,
    _4,
    _5,
    _6,
    _7,
    _8,
    _9,
    _10,
    _11,
    _12,
}

impl Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Number::_2 => write!(f, "02"),
            Number::_3 => write!(f, "03"),
            Number::_4 => write!(f, "04"),
            Number::_5 => write!(f, "05"),
            Number::_6 => write!(f, "06"),
            Number::_7 => write!(f, "07"),
            Number::_8 => write!(f, "08"),
            Number::_9 => write!(f, "09"),
            Number::_10 => write!(f, "10"),
            Number::_11 => write!(f, "11"),
            Number::_12 => write!(f, "12"),
        }
    }
}
