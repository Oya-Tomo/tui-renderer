
#[macro_use]
pub mod app;
pub mod buffer;
pub mod widget;
pub mod effect;
pub mod border;
pub mod timer;

#[cfg(test)]
mod tests {
    use std::io::{stdin, stdout, Write};
    use std::time::{self, Duration};
    use termion::{cursor};
    use termion::input::TermRead;
    use termion::raw::IntoRawMode;
    use termion::event::{Key, Event};
    use unicode_width::{UnicodeWidthStr};

    use crate::border;
    use crate::buffer::{Buffer, Rect};
    use crate::effect::{Effect, Color};
    use crate::timer::Timer;
    use crate::widget::layout::{Horizontal, Constraint, Alignment, Vertical};
    use crate::widget::widget::Widget;

    #[test]
    fn test() {
        let stdin = stdin();
        let mut stdout = stdout().into_raw_mode().unwrap();

        // write!(stdout, "{}", clear::All).unwrap();
        write!(stdout, "{}[2J", 27 as char).unwrap();
        write!(stdout, "{}", cursor::Goto(1, 1)).unwrap();

        for event in stdin.events() {
            match event.unwrap() {
                Event::Key(Key::Ctrl('c')) => {
                    return ;
                }
                _ => {}
            }

            write!(stdout, "{}", termion::clear::All).unwrap();
            // print!("\x1B[2J");
            // std::process::Command::new("clear").status().unwrap();
            
            write!(stdout, "{}", cursor::Goto(1, 1)).unwrap();

            let (width, height) = termion::terminal_size().unwrap();
            for y in 1..height + 1 {
                for x in 1..width {
                    write!(stdout, "{}", termion::cursor::Goto(x, y)).unwrap();
                    write!(stdout, "{}", x).unwrap();
                }
            }
            stdout.flush().unwrap();
        }
    }

    #[test]
    fn buf_test() {
        let start = time::Instant::now();

        // let (width, height) = termion::terminal_size().unwrap();
        // let mut buf = Buffer::new(width as usize, height as usize);
        let mut buf = Buffer::new(100, 30);
        let w_buf = vec![
            "╭────────╮".to_string(),
            "│   あ  │".to_string(),
            "│        │".to_string(),
            "╰────────╯".to_string(),
        ];

        let rect1 = Rect::new(0, 0, 10, 4);
        let rect2 = Rect::new(10, 0, 10, 4);
        let rect3 = Rect::new(0, 4, 10, 4);
        let rect4 = Rect::new(10, 4, 10, 4);

        buf.push_rect_buffer(w_buf.clone(), vec![], rect1);
        buf.push_rect_buffer(w_buf.clone(), vec![], rect2);
        buf.push_rect_buffer(w_buf.clone(), vec![], rect3);
        buf.push_rect_buffer(w_buf.clone(), vec![], rect4);

        for l in &buf.get_buffer() {
            print!("\x1b[42m");
            print!("{}", l);
            print!("\x1b[m");
            println!("|  len : {}", UnicodeWidthStr::width(l.as_str()));
        }

        // sleep(Duration::from_secs(1));
        let end = start.elapsed();
        println!("run time : {}", end.as_micros() as f32 / 1000000.0)
    }

    #[test]
    fn rep_test() {
        let line = " ".to_string().repeat(120);
        let start1 = time::Instant::now();
        let mut buf1 = std::iter::repeat(line.clone()).take(30);
        let end1 = start1.elapsed().as_micros();

        let start2 = time::Instant::now();
        let mut buf2 = vec![];
        for i in 0..30 {
            buf2.push(line.clone());
        }
        let end2 = start2.elapsed().as_micros();

        println!("1 : {}ms", end1 as f32 / 1000000.0);
        println!("2 : {}ms", end2 as f32 / 1000000.0);
    }

    #[test]
    fn ly_test() {
        // let frame_dur = Duration::new(0, 10_000_000);
        // let mut timer = Timer::new(frame_dur);

        // let start = time::Instant::now();
        // for i in 0..1000 {


        //     timer.frame_wait();
        // }

        // let end = start.elapsed();

        // println!("run time : {}s", end.as_nanos() as f64 / 1_000_000_000.0);
        // println!("run time : {}s / frame", end.as_nanos() as f64 / 1_000_000_000.0 / 1000.0)

        print!("\x1b[2J");
        let mut ver_lay = Vertical::new(
            Constraint::percentage(100, 200, 0),
            Alignment::Start
        ).add(Box::from(
            Horizontal::new(
                Constraint::constant(3),
                Alignment::Start
            )
            .set_border(Option::from(border::SINGLE_ROUNDED))
            .set_color(Some(Color::from_rgb(255, 255, 255)), Some(Color::from_rgb(100, 100, 100)))
        ))
        .add(Box::from(
            Horizontal::new(
                Constraint::percentage(100, 200, 5),
                Alignment::Start
            )
            .set_border(Some(border::SINGLE_ROUNDED))
            .set_color(Some(Color::from_rgb(255, 255, 255)), Some(Color::from_rgb(80, 80, 80)))
            .add(Box::from(
                Vertical::new(Constraint::constant(20), Alignment::Start)
                .set_border(Some(border::SINGLE_ROUNDED))
                .set_color(Some(Color::from_rgb(255, 255, 255)), Some(Color::from_rgb(120, 120, 120)))
            ))
        ))
        .add(Box::from(
            Horizontal::new(
                Constraint::constant(3),
                Alignment::End
            )
            .set_border(Option::from(border::SINGLE))
            .set_color(Some(Color::from_rgb(255, 255, 255)), Some(Color::from_rgb(100, 100, 100)))
        ));
        
        let (width, height) = termion::terminal_size().unwrap();
        let mut buf = Buffer::new(width as usize, height as usize);
        let rect = Rect::new(0, 0, width as usize, height as usize);

        ver_lay.write_buffer(&mut buf, rect);

        let text = buf.get_buffer();

        for l in 0..text.len() {
            print!("\x1b[{};1H", l + 1);
            print!("\x1b[2K");
            print!("{}", text[l].as_str());
            if l != text.len() - 1 {
                print!("\n");
            }
        }
    }

    #[test]
    fn eff_test() {
        let front_col = Effect::front_color(Color::from_rgb(0, 0, 0), 0);
        let back_col = Effect::back_color(Color::from_rgb(30, 170, 150), 0);
        println!(
            "{}{}{}|{}|  a |{}{}{} |{} ",
            Effect::reset(0).to_string().as_str(),
            front_col.to_string().as_str(),
            back_col.to_string().as_str(),
            Effect::reset(1).to_string().as_str(),
            Effect::reset(1).to_string().as_str(),
            front_col.to_string().as_str(),
            back_col.to_string().as_str(),
            Effect::reset(1).to_string().as_str(),
        );
    }
}

// reference
// - https://ja.wikipedia.org/wiki/%E7%BD%AB%E7%B7%9A%E7%B4%A0%E7%89%87