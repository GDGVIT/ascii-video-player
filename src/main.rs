use opencv::prelude::*;
use opencv::{highgui, imgproc, videoio, core, Result};

fn map_range(from_range: (i32, i32), to_range: (i32, i32), s: i32) -> i32 {
    to_range.0 + (s - from_range.0) * (to_range.1 - to_range.0) / (from_range.1 - from_range.0)
}

fn encode_frame(frame: Mat, table: &str, table_len: usize) -> Result<String> {
    let mut out_frame = String::new();

    let rows = frame.rows();
    let cols = frame.cols();

    for row in 0..rows {
        for col in 0..cols {
            let pixel = frame.at_2d::<u8>(row, col)?;

            let new_pixel = map_range((0, 255), (0, (table_len - 1) as i32), *pixel as i32);
            out_frame.push(table.as_bytes()[new_pixel as usize] as char);
        }

        out_frame.push_str("\n");
    }

    Ok(out_frame)
}

fn main() -> Result<()> {
    let ascii_table =
        "     .'`^\",:;Il!i><~+_-?][}{1)(|\\/tfjrxnuvczXYUJCLQ0OZmwqpdbkhao*#MW&8%B@$";
    let ascii_table_len = ascii_table.len();

    let term_size = termion::terminal_size().unwrap();

    let window = "Video";
    highgui::named_window(window, 1)?;

    let mut cam = videoio::VideoCapture::from_file("bad-apple.webm", videoio::CAP_ANY)?;

    if !cam.is_opened()? {
        panic!("Unable to open video file");
    }

    loop {
        let mut frame = Mat::default();
        cam.read(&mut frame)?;

        if frame.size()?.width > 0 {
            let mut smaller = Mat::default();
            imgproc::resize(&frame, &mut smaller, core::Size::new(term_size.0.into(), term_size.1.into()), 0.0, 0.0, imgproc::INTER_AREA)?;

            let mut gray = Mat::default();
            imgproc::cvt_color_def(&smaller, &mut gray, imgproc::COLOR_BGR2GRAY)?;

            highgui::imshow(window, &gray)?;

            println!("{}", encode_frame(gray, ascii_table, ascii_table_len)?);
        }

        if highgui::wait_key(10)? > 0 {
            break;
        }
    }
    Ok(())
}
