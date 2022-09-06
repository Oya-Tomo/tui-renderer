use std::iter;

use crate::{effect::{Effect, EffectType, Color}, border::BorderLine};
use unicode_width::{UnicodeWidthChar};

pub struct Buffer {
    width: usize,
    height: usize,
    text: Vec<String>,
    pub effects: Vec<Vec<Effect>>
}

impl Buffer {
    pub fn new(width: usize, height: usize) -> Self {
        let text = iter::repeat(" ".to_string().repeat(width)).take(height).into_iter().collect();
        let effects: Vec<Vec<Effect>> = iter::repeat(Vec::new()).take(height).into_iter().collect();
        Buffer { width: width, height: height, text: text, effects: effects }
    }

    pub fn push_rect_buffer(&mut self, buf: Vec<String>, effects: Vec<Vec<Effect>>, rect: Rect) {
        for y in 0..rect.height {
            let line_vec: Vec<char> = self.text[rect.y + y].chars().collect();

            let start_width = rect.x;
            let mut crt_len_start = 0;
            let mut start_idx = 0;
            let mut start_term = "";

            let end_width = self.width - (rect.x + rect.width);
            let mut crt_len_end = 0;
            let mut end_idx = 0;
            let mut end_term = "";

            for i in 0..line_vec.len() {
                let c_width = line_vec[i].width().unwrap_or(2);

                if crt_len_start + c_width > start_width {
                    start_idx = i;
                    if c_width == 2 && crt_len_start + c_width == start_width + 1{
                        start_term = " ";
                    }
                    break;
                }
                crt_len_start += c_width;
            }

            for i in (0..line_vec.len()).rev() {
                let c_width = line_vec[i].width().unwrap_or(2);

                if crt_len_end + c_width > end_width {
                    end_idx = i + 1;
                    if c_width == 2 && crt_len_end + c_width == end_width + 1{
                        end_term = " ";
                    }
                    break;
                }
                crt_len_end += c_width;
            }

            self.text[rect.y + y] = line_vec[..start_idx].into_iter().collect::<String>() +
                                    start_term +
                                    &buf[y] +
                                    end_term +
                                    &line_vec[end_idx..].into_iter().collect::<String>();
            
            // push effect vec to buffer !!
            let mut ori_eff_vec = self.effects[rect.y + y].clone();

            let mut eff_vec_start: Vec<Effect> = vec![];
            let mut eff_vec_end: Vec<Effect> = vec![];
            
            for i in 0..ori_eff_vec.len() {
                if ori_eff_vec[i].pos >= rect.x {
                    eff_vec_start = ori_eff_vec[..i].to_vec();
                    break;
                }
            }

            let mut child_effs = effects[y].clone();
            for i in 0..child_effs.len() {
                child_effs[i].pos += rect.x;
            }

            for i in (0..ori_eff_vec.len()).rev() { 
                if ori_eff_vec[i].kind == EffectType::Reset && ori_eff_vec[i].pos <= rect.x + rect.width {
                    eff_vec_end = ori_eff_vec[i + 1..].to_vec();
                    break;

                } else if ori_eff_vec[i].pos <= rect.x + rect.width {
                    ori_eff_vec[i].pos = rect.x + rect.width;
                }
                if i == 0 {
                    eff_vec_end = ori_eff_vec.clone();
                }
            }

            self.effects[rect.y + y] = vec![];
            self.effects[rect.y + y].append(&mut eff_vec_start.clone());
            self.effects[rect.y + y].append(&mut child_effs.clone());
            self.effects[rect.y + y].append(&mut eff_vec_end.clone());
        }
    }

    pub fn get_buffer(&self) -> Vec<String> {
        let mut view_buf = vec![];

        for y in 0..self.text.len() {
            let line_chars = self.text[y].chars().collect::<Vec<char>>();
            let line_effs = self.effects[y].clone();
            let mut line_text = "".to_string();
            let mut crt_line_len = 0;
            let mut eff_idx = 0;

            for ch in line_chars {
                for i in eff_idx..line_effs.len() {
                    if line_effs[i].pos == crt_line_len {
                        line_text += &line_effs[i].to_string();
                    } else if line_effs[i].pos > crt_line_len {
                        eff_idx = i;
                        break;
                    }
                }
                line_text += &ch.to_string();
                crt_line_len += ch.width().unwrap_or(2);
            }

            for i in eff_idx..line_effs.len() {
                if line_effs[i].pos == crt_line_len {
                    line_text += &line_effs[i].to_string();
                } else if line_effs[i].pos > crt_line_len {
                    eff_idx = i;
                }
            }

            view_buf.push(line_text);
        }

        return view_buf;

    }
}

#[derive(Clone, Copy)]
pub struct Rect {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
}

impl Rect {
    pub fn new(x: usize, y: usize, width: usize, height: usize) -> Self {
        Rect { x: x, y: y, width: width, height: height }
    }

    pub fn can_draw_border(&self) -> bool {
        if self.width >= 3 && self.height >= 3 {
            return true;
        } else {
            return false;
        }
    }

    pub fn make_bordered_rect(&self, line: Option<BorderLine>) -> Rect {
        if self.can_draw_border() && line.is_some() {
            let mut rect = self.clone();
            rect.x += 1;
            rect.y += 1;
            rect.width -= 2;
            rect.height -= 2;
            return rect;
        } else {
            return self.clone();
        }
    }

    pub fn make_border_buf(&self, line: Option<BorderLine>) -> Vec<String> {
        if line.is_none() || !self.can_draw_border() {
            return iter::repeat(" ".to_string().repeat(self.width)).take(self.height).into_iter().collect();
        } else {
            let border = line.unwrap();
            let mut buf: Vec<String> = iter::repeat(
                border.l.to_string() +
                &" ".to_string().repeat(self.width - 2) +
                &border.r.to_string()
            ).take(self.height - 2).into_iter().collect();

            buf.insert(
                0,
                border.lt.to_string() +
                &border.t.to_string().repeat(self.width - 2) +
                &border.rt.to_string()
            );
            buf.push(
                border.lb.to_string() +
                &border.b.to_string().repeat(self.width - 2) +
                &border.rb.to_string()
            );

            return buf;
        }
    }

    pub fn make_color_eff_vec(&self, front: Option<Color>, back: Option<Color>) -> Vec<Vec<Effect>> {
        let mut eff_vec = vec![];
        if let Some(front) = front {
            eff_vec.push(Effect::front_color(front, 0));
        }
        if let Some(back) = back {
            eff_vec.push(Effect::back_color(back, 0));
        }

        let mut line_eff_vec = vec![Effect::reset(0)];
        line_eff_vec.append(&mut eff_vec);
        line_eff_vec.push(Effect::reset(self.width));

        return iter::repeat(line_eff_vec).take(self.height).into_iter().collect();
    }
}