
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct BorderLine {
    pub l: char,
    pub r: char,
    pub t: char,
    pub b: char,
    pub lt: char, // left top
    pub rt: char, // right top
    pub lb: char, // left bottom
    pub rb: char, // right bottom
}

impl BorderLine {
    pub fn new(
        left: char,
        right: char,
        top: char,
        bottom: char,
        left_top: char,
        right_top: char,
        left_bottom: char,
        right_bottom: char,
    ) -> Self {
        BorderLine {
            l: left,
            r: right,
            t: top,
            b: bottom,
            lt: left_top,
            rt: right_top,
            lb: left_bottom,
            rb: right_bottom
        }
    }
}

pub const SPACE: BorderLine = BorderLine {
    l: ' ',
    r: ' ',
    t: ' ',
    b: ' ',
    lt: ' ',
    rt: ' ',
    lb: ' ',
    rb: ' ',
};

pub const SINGLE: BorderLine = BorderLine {
    l: '│',
    r: '│',
    t: '─',
    b: '─',
    lt: '┌',
    rt: '┐',
    lb: '└',
    rb: '┘',
};

pub const SINGLE_ROUNDED: BorderLine = BorderLine {
    l: '│',
    r: '│',
    t: '─',
    b: '─',
    lt: '╭',
    rt: '╮',
    lb: '╰',
    rb: '╯',
};

pub const DOTTED: BorderLine = BorderLine {
    l: '┆',
    r: '┆',
    t: '┄',
    b: '┄',
    lt: '┌',
    rt: '┐',
    lb: '└',
    rb: '┘',
};

pub const DOTTED_ROUNDED: BorderLine = BorderLine {
    l: '┆',
    r: '┆',
    t: '┄',
    b: '┄',
    lt: '╭',
    rt: '╮',
    lb: '╰',
    rb: '╯',
};

pub const DOUBLE: BorderLine = BorderLine {
    l: '║',
    r: '║',
    t: '═',
    b: '═',
    lt: '╔',
    rt: '╗',
    lb: '╚',
    rb: '╝',
};