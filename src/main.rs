use std::time;
use std::{error::Error, thread};

struct Frame {
    pixel: Vec<Vec<char>>,
    height: usize,
    width: usize,
}

impl Frame {
    fn new(height: usize, width: usize) -> Self {
        Frame {
            pixel: vec![vec![' '; width]; height],
            height,
            width,
        }
    }

    fn override_at_with_frame(&mut self, x: i32, y: i32, frame: &Frame) -> Result<(), &str> {
        for (line_index, line) in frame.pixel.iter().enumerate() {
            if line_index as i32 + y > self.height as i32 - 1 || line_index as i32 + y < 0 {
                continue;
            }
            for (c_index, c) in line.iter().enumerate() {
                if c_index as i32 + x > self.width as i32 - 1 || c_index as i32 + x < 0 {
                    continue;
                }
                self.pixel[(line_index as i32 + y) as usize][(c_index as i32 + x) as usize] = *c;
            }
        }
        Ok(())
    }

    fn override_at_with_string(&mut self, x: i32, y: i32, str: &str) -> Result<(), &str> {
        let mut frame: Frame = Frame::new(1, str.len());

        for (c_index, c) in str.as_bytes().iter().enumerate() {
            frame.pixel[0][c_index] = *c as char;
        }

        self.override_at_with_frame(x, y, &frame)?;
        Ok(())
    }

    fn print(&self) {
        for line in &self.pixel {
            for c in line {
                print!("{}", *c);
            }
            print!("\n");
        }
    }
}

struct Movie {
    frames_per_second: u32,
    width: usize,
    height: usize,
    frames: Vec<Frame>,
}

impl Movie {
    fn new(frames_per_second: u32, height: usize, width: usize) -> Self {
        Movie {
            frames_per_second,
            height,
            width,
            frames: vec![],
        }
    }

    fn add_frame(&mut self, frame: Frame) -> Result<(), &str> {
        if self.height != frame.height || self.width != frame.width {
            return Err("Wrong size frame");
        }

        self.frames.push(frame);

        Ok(())
    }

    fn play(&self) {
        for frame in self.frames.iter() {
            print!("\x1B[2J\x1B[1;1H");
            frame.print();
            thread::sleep(time::Duration::from_millis(
                (1000 / self.frames_per_second).into(),
            ));
        }
    }
}
fn main() {
    let mut movie: Movie = Movie::new(10, 10, 10);

    let mut frame2 = Frame::new(4, 4);
    frame2.override_at_with_string(0, 0, "XXXX");
    frame2.override_at_with_string(0, 1, "XXXX");
    frame2.override_at_with_string(0, 2, "XXXX");
    frame2.override_at_with_string(0, 3, "XXXX");
    for i in 0..20 {
        let mut frame = Frame::new(10, 10);
        frame.override_at_with_frame(i - 2, i - 2, &frame2);
        let _ = movie.add_frame(frame);
    }

    movie.play();
}
