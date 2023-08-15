use std::fs::File;
use std::io::Read;
use std::thread;
use std::time;
use std::time::Duration;

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

    fn from_file(height: usize, width: usize, file_path: &str) -> Result<Self, std::io::Error> {
        let mut frame = Frame::new(height, width);

        let mut file = File::open(file_path)?;

        let mut file_contents = String::new();
        file.read_to_string(&mut file_contents)?;

        let lines: Vec<String> = file_contents.lines().map(String::from).collect();

        for (index, line) in lines.iter().enumerate() {
            frame
                .override_at_with_string(0, index as i32, line)
                .unwrap_or_default();
        }

        Ok(frame)
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

    fn override_at_with_movie(&mut self, x: i32, y: i32, movie: &Movie) -> Result<(), &str> {
        if self.frames_per_second != movie.frames_per_second {
            return Err("Wrong frames_per_second");
        }
        let mut i: u32 = 0;
        for frame in self.frames.iter_mut() {
            let _ = frame.override_at_with_frame(x, y, &movie.frames[i as usize]);
            i += 1;
            if i >= movie.frames.len() as u32 {
                i = 0;
            }
        }

        Ok(())
    }
    fn override_at_with_movie_start_end(
        &mut self,
        x: i32,
        y: i32,
        movie: &Movie,
        range: std::ops::Range<u32>,
    ) -> Result<(), &str> {
        if self.frames_per_second != movie.frames_per_second {
            return Err("Wrong frames_per_second");
        }

        let copied_frame_indexes: Vec<u32> = range.collect();

        let mut i: u32 = 0;
        for (frame_index, frame) in self.frames.iter_mut().enumerate() {
            if copied_frame_indexes.contains(&(frame_index as u32)) {
                let _ = frame.override_at_with_frame(x, y, &movie.frames[i as usize]);
                i += 1;
                if i >= movie.frames.len() as u32 {
                    i = 0;
                }
            }
        }
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
    match Frame::from_file(3, 6, "boy.txt") {
        Ok(frame) => frame.print(),
        Err(_) => {}
    }
}
