use tui_renderer::{buffer::{Buffer, Rect}, widget::{layout::{Vertical, Constraint, Alignment, Horizontal}, widget::Widget}, effect::Color};

fn main() {
    let (width, height) = termion::terminal_size().unwrap();
    let mut buf = Buffer::new(width as usize, height as usize);
    let rect = Rect::new(0, 0, width as usize, height as usize);

    let mut ui = Vertical::new(Constraint::percentage(100, 1000, 0), Alignment::Start)
                        .add(Box::from(
                            Horizontal::new(Constraint::constant(1), Alignment::Start)
                            .set_color(Some(Color::from_rgb(255, 255, 255)), Some(Color::from_rgb(80, 80, 80)))
                        ))
                        .add(Box::from(
                            Horizontal::new(Constraint::percentage(100, 1000, 10), Alignment::Start)
                            .add(Box::from(
                                Vertical::new(Constraint::constant(5), Alignment::Start)
                                .set_color(Some(Color::from_rgb(255, 255, 255)), Some(Color::from_rgb(90, 90, 90)))
                            ))
                            .add(Box::from(
                                Vertical::new(Constraint::constant(20), Alignment::Start)
                                .set_color(Some(Color::from_rgb(255, 255, 255)), Some(Color::from_rgb(70, 70, 70)))
                            ))
                            .add(Box::from(
                                Vertical::new(Constraint::percentage(100, 300, 10), Alignment::Start)
                                .set_color(Some(Color::from_rgb(255, 255, 255)), Some(Color::from_rgb(50, 50, 50)))
                            ))
                        ))
                        .add(Box::from(
                            Horizontal::new(Constraint::constant(1), Alignment::End)
                            .set_color(Some(Color::from_rgb(255, 255, 255)), Some(Color::from_rgb(70, 90, 255)))
                        ));
    
    ui.write_buffer(&mut buf, rect);

    for l in buf.get_buffer() {
        println!("{}", l.as_str());
    }
}