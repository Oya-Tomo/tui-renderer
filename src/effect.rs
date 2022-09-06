#[derive(Clone)]
pub struct Color {
    pub code: String
}

impl Color {
    pub fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Color { code: format!("2;{};{};{}m", r, g, b) }
    }

    pub fn from_rgb_8bit(r: u8, g: u8, b: u8) -> Self {
        assert!(r <= 5, "The red color value is too big. (r <= 5)");
        assert!(g <= 5, "The green color value is too big. (g <= 5)");
        assert!(b <= 5, "The blue color value is too big. (b <= 5)");
        Color { code: format!("5;{}m", 16 + r * 36 + g * 6 + b) }
    }

    pub fn gray(brightness: u8) -> Self {
        assert!(brightness <= 23, "The brightness value is too big. (0 - 23)");
        Color { code: format!("5;{}m", 232 + brightness) }
    }

    pub fn to_front_seq(&self) -> String {
        return "\x1b[38;".to_string() + &self.code;
    }

    pub fn to_back_seq(&self) -> String {
        return "\x1b[48;".to_string() + &self.code;
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum EffectType {
    Reset,
    Bold,
    Thin,
    Italic,
    UnderLine,
    Bling,
    FastBling,
    Invert,
    Hide,
    Cancel,
    FrontColor,
    BackColor
}

#[derive(Clone)]
pub struct Effect {
    pub kind: EffectType,
    pub pos: usize,
    seq: String
}

impl Effect {
    pub fn reset(pos: usize) -> Self {
        Effect { kind: EffectType::Reset, pos: pos, seq: "\x1b[0m".to_string()}
    }

    pub fn bold(pos: usize) -> Self {
        Effect { kind: EffectType::Bold, pos: pos, seq: "\x1b[1m".to_string() }
    }

    pub fn thin(pos: usize) -> Effect {
        Effect { kind: EffectType::Thin, pos: pos, seq: "\x1b[2m".to_string() }
    }

    pub fn italic(pos: usize) -> Effect {
        Effect { kind: EffectType::Italic, pos: pos, seq: "\x1b[3m".to_string() }
    }

    pub fn under_line(pos: usize) -> Effect {
        Effect { kind: EffectType::UnderLine, pos: pos, seq: "\x1b[4m".to_string() }
    }

    pub fn bling(pos: usize) -> Effect {
        Effect { kind: EffectType::Bling, pos: pos, seq: "\x1b[5m".to_string() }
    }

    pub fn fast_bling(pos: usize) -> Effect {
        Effect { kind: EffectType::FastBling, pos: pos, seq: "\x1b[6m".to_string() }
    }

    pub fn invert(pos: usize) -> Effect {
        Effect { kind: EffectType::Invert, pos: pos, seq: "\x1b[7m".to_string() }
    }

    pub fn hide(pos: usize) -> Effect {
        Effect { kind: EffectType::Hide, pos: pos, seq: "\x1b[8m".to_string() }
    }

    pub fn cancel(pos: usize) -> Effect {
        Effect { kind: EffectType::Cancel, pos: pos, seq: "\x1b[9m".to_string() }
    }

    pub fn front_color(color: Color, pos: usize) -> Self {
        Effect { kind: EffectType::FrontColor, pos: pos, seq: color.to_front_seq() }
    }

    pub fn back_color(color: Color, pos: usize) -> Self {
        Effect { kind: EffectType::BackColor, pos: pos, seq: color.to_back_seq() }
    }
}

impl ToString for Effect {
    fn to_string(&self) -> String {
        return self.seq.clone();
    }
}