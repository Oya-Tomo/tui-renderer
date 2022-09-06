use std::iter;

use crate::{border::BorderLine, buffer::Rect, effect::Color};
use super::widget::Widget;

#[derive(Clone, PartialEq, Eq)]
pub enum SizeType {
    Percentage,
    Constant,
}

#[derive(Clone)]
pub struct Constraint {
    pub size_type: SizeType,
    pub size: usize,
    pub min_length: usize,
    pub max_length: usize
}

impl Constraint {
    pub fn percentage(size: usize, max_length: usize, min_length: usize) -> Self {
        Constraint { size_type: SizeType::Percentage, size: size, min_length: min_length, max_length: max_length}
    }

    pub fn constant(size: usize) -> Self {
        Constraint { size_type: SizeType::Constant, size: size, min_length: size, max_length: size}
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Alignment {
    Start,
    End,
}

pub struct Horizontal {
    constraint: Constraint,
    alignment: Alignment,
    widgets: Vec<Box<dyn Widget>>,
    border: Option<BorderLine>,
    front_color: Option<Color>,
    back_color: Option<Color>,
}

impl Horizontal {
    pub fn new(constraint: Constraint, alignment: Alignment) -> Self {
        Horizontal { constraint: constraint, alignment: alignment, widgets: vec![], border: None, front_color: None, back_color: None }
    }

    pub fn add(mut self, widget: Box<dyn Widget>) -> Self {
        self.widgets.push(widget);
        return self;
    }

    pub fn set_border(mut self, line: Option<BorderLine>) -> Self {
        self.border = line;
        return self;
    }

    pub fn set_color(mut self, front: Option<Color>, back: Option<Color>) -> Self {
        self.front_color = front;
        self.back_color = back;
        return self;
    }
}

impl Widget for Horizontal {
    fn get_constraint(&self) -> Constraint {
        return self.constraint.clone();
    }

    fn get_alignment(&self) -> Alignment {
        return self.alignment.clone();
    }

    fn write_buffer(&mut self, buffer: &mut crate::buffer::Buffer, rect: Rect) {
        let view_buf = rect.make_border_buf(self.border);
        let veiw_effs = rect.make_color_eff_vec(self.front_color.clone(), self.back_color.clone());
        buffer.push_rect_buffer(view_buf, veiw_effs, rect);

        let child_area = rect.make_bordered_rect(self.border);
        let mut rem_width = child_area.width.clone();
        let mut child_widths: Vec<usize> = iter::repeat(0).take(self.widgets.len()).into_iter().collect();

        let mut child_idx_start = vec![];
        let mut child_idx_end = vec![];

        for i in 0..self.widgets.len() {
            let constraint = self.widgets[i].get_constraint();
    
            if constraint.size_type == SizeType::Constant {
                let align = self.widgets[i].get_alignment();

                if rem_width >= constraint.min_length {
                    child_widths[i] = constraint.min_length;
                    rem_width -= constraint.min_length;

                    if align == Alignment::Start {
                        child_idx_start.push(i);
                    } else {
                        child_idx_end.push(i);
                    }
                }
            }
        }
        let percent_width = rem_width.clone();
        for i in 0..self.widgets.len() {
            let constraint = self.widgets[i].get_constraint();

            if constraint.size_type == SizeType::Percentage {
                let width = percent_width * constraint.size / 100;
                let align = self.widgets[i].get_alignment();

                if rem_width >= width && width >= constraint.max_length {
                    child_widths[i] = constraint.max_length;
                    rem_width -= constraint.max_length;
                    
                    if align == Alignment::Start {
                        child_idx_start.push(i);
                    } else {
                        child_idx_end.push(i);
                    }

                } else if rem_width >= width && width >= constraint.min_length {
                    child_widths[i] = width;
                    rem_width -= width;

                    if align == Alignment::Start {
                        child_idx_start.push(i);
                    } else {
                        child_idx_end.push(i);
                    }

                } else if rem_width >= constraint.min_length {
                    child_widths[i] = constraint.min_length;
                    rem_width -= constraint.min_length;

                    if align == Alignment::Start {
                        child_idx_start.push(i);
                    } else {
                        child_idx_end.push(i);
                    }
                }
            }
        }

        let mut space_start = 0;
        let mut space_end = 0;

        for i in child_idx_start {
            let child_rect = Rect::new(
                child_area.x + space_start,
                child_area.y,
                child_widths[i],
                child_area.height
            );
            self.widgets[i].write_buffer(buffer, child_rect);
            space_start += child_widths[i];
        }

        for i in child_idx_end.iter().rev() {
            let child_rect = Rect::new(
                child_area.x + child_area.width - space_end - child_widths[i.clone()],
                child_area.y,
                child_widths[i.clone()],
                child_area.height
            );
            self.widgets[i.clone()].write_buffer(buffer, child_rect);
            space_end += child_widths[i.clone()];
        }


    }
}

pub struct Vertical {
    constraint: Constraint,
    alignment: Alignment,
    widgets: Vec<Box<dyn Widget>>,
    border: Option<BorderLine>,
    front_color: Option<Color>,
    back_color: Option<Color>,
}

impl Vertical {
    pub fn new(constraint: Constraint, alignment: Alignment) -> Self {
        Vertical { constraint: constraint, alignment: alignment, widgets: vec![], border: None, front_color: None, back_color: None }
    }

    pub fn add(mut self, widget: Box<dyn Widget>) -> Self {
        self.widgets.push(widget);
        return self;
    }

    pub fn set_border(mut self, line: Option<BorderLine>) -> Self {
        self.border = line;
        return self;
    }

    pub fn set_color(mut self, front: Option<Color>, back: Option<Color>) -> Self {
        self.front_color = front;
        self.back_color = back;
        return self;
    }
}

impl Widget for Vertical {
    fn get_constraint(&self) -> Constraint {
        return self.constraint.clone();
    }

    fn get_alignment(&self) -> Alignment {
        return self.alignment.clone();
    }

    fn write_buffer(&mut self, buffer: &mut crate::buffer::Buffer, rect: Rect) {
        let view_buf = rect.make_border_buf(self.border);
        let veiw_effs = rect.make_color_eff_vec(self.front_color.clone(), self.back_color.clone());
        buffer.push_rect_buffer(view_buf, veiw_effs, rect);

        let child_area = rect.make_bordered_rect(self.border);
        let mut rem_height = child_area.height.clone();
        let mut child_heights: Vec<usize> = iter::repeat(0).take(self.widgets.len()).into_iter().collect();

        let mut child_idx_start = vec![];
        let mut child_idx_end = vec![];

        for i in 0..self.widgets.len() {
            let constraint = self.widgets[i].get_constraint();

            if constraint.size_type == SizeType::Constant {
                let align = self.widgets[i].get_alignment();

                if rem_height >= constraint.min_length {
                    child_heights[i] = constraint.min_length;
                    rem_height -= constraint.min_length;

                    if align == Alignment::Start {
                        child_idx_start.push(i);
                    } else {
                        child_idx_end.push(i);
                    }
                }
            }
        }
        let percent_height = rem_height.clone();
        for i in 0..self.widgets.len() {
            let constraint = self.widgets[i].get_constraint();

            if constraint.size_type == SizeType::Percentage {
                let height = percent_height * constraint.size / 100;
                let align = self.widgets[i].get_alignment();

                if rem_height >= height && height >= constraint.max_length {
                    child_heights[i] = constraint.max_length;
                    rem_height -= constraint.max_length;

                    if align == Alignment::Start {
                        child_idx_start.push(i);
                    } else {
                        child_idx_end.push(i)
                    }

                } else if rem_height >= height && height >= constraint.min_length {
                    child_heights[i] = height;
                    rem_height -= height;

                    if align == Alignment::Start {
                        child_idx_start.push(i);
                    } else {
                        child_idx_end.push(i);
                    }

                } else if rem_height >= constraint.min_length {
                    child_heights[i] = constraint.min_length;
                    rem_height -= constraint.min_length;

                    if align == Alignment::Start {
                        child_idx_start.push(i);
                    } else {
                        child_idx_end.push(i);
                    }
                }
            }
        }

        let mut space_start = 0;
        let mut space_end = 0;

        for i in child_idx_start {
            let child_rect = Rect::new(
                child_area.x,
                child_area.y + space_start,
                child_area.width,
                child_heights[i],
            );
            self.widgets[i].write_buffer(buffer, child_rect);
            space_start += child_heights[i];
        }

        for i in child_idx_end.iter().rev() {
            let child_rect = Rect::new(
                child_area.x,
                child_area.y + child_area.height - space_end - child_heights[i.clone()],
                child_area.width,
                child_heights[i.clone()],
            );
            self.widgets[i.clone()].write_buffer(buffer, child_rect);
            space_end += child_heights[i.clone()];
        }
    }
}