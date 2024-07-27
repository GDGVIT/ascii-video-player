use opencv::core::VecN;
use opencv::prelude::*;
use opencv::{core, imgproc, videoio, Result};
use std::sync::{mpsc, Arc};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use std::{thread, time};


use crossterm::event::{self, Event, KeyCode, KeyEvent};
use crossterm::{
    cursor::{Hide, Show, MoveTo},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
};
use crossterm::terminal;
use std::io;

fn map_range(from_range: (i32, i32), to_range: (i32, i32), s: i32) -> i32 {
    to_range.0 + (s - from_range.0) * (to_range.1 - to_range.0) / (from_range.1 - from_range.0)
}

fn find_colors(frame: &Mat, gray: &Mat, table: &str, table_len: usize) -> Result<String> {
    let mut out_colors = String::with_capacity((frame.rows() * frame.cols()) as usize * 4);

    let rows = frame.rows();
    let cols = frame.cols();

    for row in 0..rows {
        for col in 0..cols {
            let pixel = frame.at_2d::<VecN<u8, 3>>(row, col)?;

            let gray_pixel = gray.at_2d::<u8>(row, col)?;
            let new_pixel = map_range((0, 255), (0, (table_len - 1) as i32), *gray_pixel as i32);
            out_colors.push_str(&*format!(
                "\x1b[38;2;{};{};{}m{}",
                pixel[2],
                pixel[1],
                pixel[0],
                table.as_bytes()[new_pixel as usize] as char
            ))
        }

        out_colors.push_str("\r\n");
    }

    Ok(out_colors)
}

fn main() -> Result<()> {
    enable_raw_mode().unwrap();

    let mut stdout = io::stdout();

    execute!(stdout, terminal::EnterAlternateScreen).unwrap();

    let mut is_paused = false;
    let mut time_multiplier = 1.0;

    let ascii_table =
        "     .'`^\",:;Il!i><~+_-?][}{1)(|\\/tfjrxnuvczXYUJCLQ0OZmwqpdbkhao*#MW&8%B@$";
    let ascii_table_len = ascii_table.len();

    let term_size = crossterm::terminal::size().unwrap();

    let mut cam = videoio::VideoCapture::from_file("baby-shark.webm", videoio::CAP_ANY)?;

    if !cam.is_opened()? {
        panic!("Unable to open video file");
    }

    let fps = cam.get(videoio::CAP_PROP_FPS)?;
    // time b/w each frame
    let frame_delay = time::Duration::from_millis((1000.0 / fps) as u64);

    let (tx, rx) = mpsc::sync_channel(50);
    execute!(stdout, Hide).unwrap();

    let handle = thread::spawn(move || loop {
        let mut frame = Mat::default();
        cam.read(&mut frame).unwrap();

        if frame.size().unwrap().width > 0 {
            let mut smaller = Mat::default();
            imgproc::resize(
                &frame,
                &mut smaller,
                core::Size::new(term_size.0.into(), (term_size.1 - 2).into()),
                0.0,
                0.0,
                imgproc::INTER_AREA,
            )
                .unwrap();

            let mut gray = Mat::default();
            imgproc::cvt_color_def(&smaller, &mut gray, imgproc::COLOR_BGR2GRAY).unwrap();

            let Ok(_) = tx.send(find_colors(&smaller, &gray, ascii_table, ascii_table_len).unwrap()) else {
                break;
            };
        }

    });

    loop {

        if !is_paused {
            let received = rx.recv().unwrap();

            execute!(stdout, MoveTo(0, 0)).unwrap();
            execute!(stdout, Clear(ClearType::Purge)).unwrap();
            print!("{}", received);

            thread::sleep(frame_delay.mul_f32(time_multiplier));
        }

        if event::poll(Duration::from_millis(0)).unwrap() {
            match event::read().unwrap() {
                Event::Key(KeyEvent {
                    code: KeyCode::Char('q'),
                    ..
                }) => {
                    // breaking here drops rx, preventing tx from sending which then breaks the other
                    // thread
                    break;
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Char(' '),
                    ..
                }) => {
                    is_paused = !is_paused;
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Char('l'),
                    ..
                }) => {
                    time_multiplier = time_multiplier * 2.0;
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Char('j'),
                    ..
                }) => {
                    time_multiplier = time_multiplier / 2.0;
                }
                _ => {}
            }
        }
    }

    execute!(stdout, terminal::LeaveAlternateScreen).unwrap();
    execute!(stdout, Show).unwrap();
    disable_raw_mode().unwrap();
    Ok(())
}
