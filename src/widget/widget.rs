use crate::buffer::{Buffer, Rect};
use super::layout::{Constraint, Alignment};

pub trait Widget {
    fn get_constraint(&self) -> Constraint;
    fn get_alignment(&self) -> Alignment;
    fn write_buffer(&mut self, buffer: &mut Buffer, rect: Rect);
}