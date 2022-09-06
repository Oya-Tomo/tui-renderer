use unicode_width::UnicodeWidthStr;

use crate::buffer::{Buffer, Rect};
use crate::border::BorderLine;
use super::layout::Constraint;
use super::widget::Widget;

pub struct Button {
    text: String,
    constraint: Constraint
}

impl Button {
    fn new(text: String) -> Self {
        let size = text.width();
        Button {
            text: text,
            constraint: Constraint::constant(size)
        }
    }

    fn set_border(&mut self, line: Option<BorderLine>) {
        
    }
}

impl Widget for Button {
    fn get_constraint(&self) -> Constraint {
        return self.constraint.clone();
    }

    fn write_buffer(&mut self, buffer: &mut Buffer, rect: Rect) {
    }
}

