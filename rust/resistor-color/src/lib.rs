use int_enum::IntEnum;
use enum_iterator::{all, Sequence};

#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, IntEnum, Sequence)]
pub enum ResistorColor {
    Black = 0,
    Blue = 6,
    Brown = 1,
    Green = 5,
    Grey = 8,
    Orange = 3,
    Red = 2,
    Violet = 7,
    White = 9,
    Yellow = 4,
}

pub fn color_to_value(_color: ResistorColor) -> u32 {
    _color.int_value()
}

pub fn value_to_color_string(value: u32) -> String {
    if let Ok(color) = ResistorColor::from_int(value) {
        let colorstring = format!("{:?}", color);
        colorstring
    } else {
        "value out of range".to_string()
    }
}

pub fn colors() -> Vec<ResistorColor> {
    let mut unsorted = all::<ResistorColor>().collect::<Vec<ResistorColor>>();
    unsorted.sort_by(|a, b| a.int_value().cmp(&b.int_value()));
    unsorted
}
