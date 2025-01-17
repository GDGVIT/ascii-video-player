use opencv::core::VecN;
use opencv::prelude::*;
use opencv::{core, imgproc, videoio, Result};
use std::sync::mpsc;
use std::time::Duration;
use std::{thread, time};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventState};
use crossterm::terminal;
use crossterm::{
    cursor::{Hide, MoveTo, Show},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
};

use std::env;
use std::io;
use std::io::Write;
use crossterm::style::{Color, SetForegroundColor};

fn map_range(from_range: (i32, i32), to_range: (i32, i32), s: i32) -> i32 {
    //enhancing readability and handling errors
    let (from_min, from_max) = from_range;
    let (to_min, to_max) = to_range;

    //opencv::core::normalize(from_min, from_max, 0, 255, opencv::core::NORM_MINMAX, -1, None).unwrap();
    /*
    if from_min == from_max{
        panic!("Invalid from_range: start and end cannot be the same."); // highly unlikely but not impossible in case image is corrupted
    }
     */

    to_min + (s - from_min) * (to_max - to_min) / (from_max - from_min)
}

fn find_colors(frame: &Mat, gray: &Mat, table: &str, table_len: usize) -> Result<String> {
    let mut out_colors = String::with_capacity((frame.rows() * frame.cols()) as usize * 4);

    let rows = frame.rows();
    let cols = frame.cols();

    for row in 0..rows {
        for col in 0..cols {
            let pixel = frame.at_2d::<VecN<u8, 3>>(row, col)?;

            let gray_pixel = gray.at_2d::<u8>(row, col)?;
            let clamped_pixel = gray_pixel.clamp(&0, &255); // in case the image is corrupted, this clamps the value between 0 and 255
            let new_pixel = map_range((0, 255), (0, (table_len - 1) as i32), *clamped_pixel as i32);
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

fn handle_args(args: Vec<String>, video_file: &mut String){
    if args.len() < 2 {
        eprintln!("Usage: {} <video file>", args[0]);
        return;
    }
    *video_file = args[1].clone();
}

fn main() -> Result<()> {

    let mut is_paused = false;
    let mut time_multiplier = 1.0;

    let ascii_table =
        "     .'`^\",:;Il!i><~+_-?][}{1)(|\\/tfjrxnuvczXYUJCLQ0OZmwqpdbkhao*#MW&8%B@$";
    let ascii_table_len = ascii_table.len();

    let term_size = crossterm::terminal::size().unwrap();

    let args: Vec<String> = env::args().collect();
    let mut video_path = String::from("");
    handle_args(args, &mut video_path);
    let video_file = video_path.as_str();

    let mut cam = videoio::VideoCapture::from_file(video_file, videoio::CAP_ANY)?;

    if !cam.is_opened()? {
        println!("Unable to open video file: {}", video_file);
        return Ok(());
    }

    let fps = cam.get(videoio::CAP_PROP_FPS)?;
    // time b/w each frame
    let frame_delay = time::Duration::from_millis((1000.0 / fps) as u64);

    enable_raw_mode().unwrap();
    let mut stdout = io::stdout();
    execute!(stdout, terminal::EnterAlternateScreen).unwrap();

    let (tx, rx) = mpsc::sync_channel(50);
    execute!(stdout, Hide).unwrap();

    thread::spawn(move || loop {
        let mut frame = Mat::default();
        if !cam.read(&mut frame).unwrap() || frame.empty(){
            return Ok(());
        }
        cam.read(&mut frame).unwrap();

        if frame.size().unwrap().width > 0 {
            let mut smaller = Mat::default();
            imgproc::resize(
                &frame,
                &mut smaller,
                core::Size::new(term_size.0.into(), (term_size.1 - 1).into()),
                0.0,
                0.0,
                imgproc::INTER_AREA,
            )
            .unwrap();

            let mut gray = Mat::default();
            imgproc::cvt_color_def(&smaller, &mut gray, imgproc::COLOR_BGR2GRAY).unwrap();

            let Ok(_) =
                tx.send(find_colors(&smaller, &gray, ascii_table, ascii_table_len).unwrap())
            else {
                return Ok::<(), String>(());
                //break;
            };
        }
    });

    loop {

        //----------------------------
        // showing the playback speed at the bottom of the screen
        let speed_indicator = format!("{}x", 1.0/time_multiplier);

        execute!(
            stdout,
            MoveTo(0, term_size.1 - 1),
            Clear(ClearType::CurrentLine)
        ).unwrap();

        execute!(stdout, MoveTo((term_size.0 - speed_indicator.len() as u16)/2, term_size.1-1)).unwrap();
        execute!(stdout, SetForegroundColor(Color::Yellow)).unwrap();
        print!("{speed_indicator}");
        let _ = stdout.flush();
        //-----------------------------

        if !is_paused {
            match rx.recv(){
                Ok(received) => {
                    execute!(stdout, MoveTo(0, 0)).unwrap();
                    execute!(stdout, Clear(ClearType::Purge)).unwrap();
                    print!("{}", received);

                    thread::sleep(frame_delay.mul_f32(time_multiplier));
                },
                Err(_) => {
                    break;
                }
            }
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
                    code: KeyCode::Left, // more intuitive keybinding
                    //code: KeyCode::Char('l'),
                    ..
                }) => {
                    if time_multiplier<4.0{
                        time_multiplier = time_multiplier * 2.0;
                    }
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Right, // more intuitive keybinding
                    //code: KeyCode::Char('j'),
                    ..
                }) => {
                    if time_multiplier>0.25{
                        time_multiplier = time_multiplier / 2.0;
                    }
                }
                _ => {}
            }
        }
        execute!(stdout, MoveTo(0,0)).unwrap();
    }

    // provides an exit prompt before leaving the alternate screen
    let exit_prompt = String::from("VIDEO FEED ENDED. PRESS ANY KEY TO EXIT");
    execute!(stdout, MoveTo((term_size.0 - exit_prompt.len() as u16)/2, term_size.1-1)).unwrap();
    execute!(stdout, SetForegroundColor(Color::Green)).unwrap();
    print!("{}",exit_prompt);
    let _ = stdout.flush();
    loop {
        if event::poll(std::time::Duration::from_secs(1)).unwrap() {
            if let Event::Key(key_event) = event::read().unwrap() {
                match key_event {
                    _ => break
                }
            }
        }
    }
    execute!(stdout, terminal::LeaveAlternateScreen).unwrap();
    execute!(stdout, Show).unwrap();
    disable_raw_mode().unwrap();
    Ok(())
}
