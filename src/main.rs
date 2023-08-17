#[allow(unused_imports)]
use std::fs::File;
use std::io::Read;
use std::thread;
use std::time;

pub struct Frame {
    height: usize,
    width: usize,
    pixels: Vec<Vec<char>>,
}

impl Clone for Frame {
    fn clone(&self) -> Self {
        Frame {
            height: self.height,
            width: self.width,
            pixels: self
                .pixels
                .iter()
                .map(|inner_vec| inner_vec.clone())
                .collect(),
        }
    }
}

impl Frame {
    // Constructors
    pub fn new(height: usize, width: usize) -> Self {
        Frame {
            pixels: vec![vec![' '; width]; height],
            height,
            width,
        }
    }

    pub fn from_string(str: &str) -> Self {
        let mut frame = Frame::new(1, str.len());

        frame.override_with_string(str);

        frame
    }

    pub fn from_file(height: usize, width: usize, file_path: &str) -> Result<Self, std::io::Error> {
        let mut file = File::open(file_path)?;
        let mut file_contents = String::new();
        file.read_to_string(&mut file_contents)?;

        let lines: Vec<String> = file_contents.lines().map(String::from).collect();

        let mut frame = Frame::new(height, width);

        for (index, line) in lines.iter().enumerate() {
            frame.override_with_string_at_position(0, index as i32, line);
        }

        Ok(frame)
    }

    // Override
    pub fn override_with_frame(&mut self, frame: &Frame) {
        self.override_with_frame_at_position(0, 0, frame);
    }

    pub fn override_with_frame_at_position(&mut self, x: i32, y: i32, frame: &Frame) {
        for (line_index, line) in frame.pixels.iter().enumerate() {
            if line_index as i32 + y > self.height as i32 - 1 || line_index as i32 + y < 0 {
                continue;
            }
            for (c_index, c) in line.iter().enumerate() {
                if c_index as i32 + x > self.width as i32 - 1 || c_index as i32 + x < 0 {
                    continue;
                }
                self.pixels[(line_index as i32 + y) as usize][(c_index as i32 + x) as usize] = *c;
            }
        }
    }

    pub fn override_with_string(&mut self, str: &str) {
        self.override_with_string_at_position(0, 0, str);
    }

    pub fn override_with_string_at_position(&mut self, x: i32, y: i32, str: &str) {
        let mut frame: Frame = Frame::new(1, str.len());

        for (c_index, c) in str.as_bytes().iter().enumerate() {
            frame.pixels[0][c_index] = *c as char;
        }

        self.override_with_frame_at_position(x, y, &frame);
    }

    pub fn print(&self) {
        for line in &self.pixels {
            for c in line {
                print!("{}", *c);
            }
            print!("\n");
        }
    }

    fn get_width(&self) -> usize {
        self.width
    }

    fn get_height(&self) -> usize {
        self.height
    }
}

pub struct Movie {
    width: usize,
    height: usize,
    frames: Vec<Frame>,
}

#[allow(dead_code)]
impl Movie {
    // Constructors
    pub fn new(height: usize, width: usize) -> Self {
        Movie {
            height,
            width,
            frames: vec![],
        }
    }

    pub fn new_blank(height: usize, width: usize, frame_amount: usize) -> Self {
        let mut movie = Movie::new(height, width);
        let blank_frame = Frame::new(height, width);
        for _i in 0..frame_amount {
            movie.add_frame(&blank_frame);
        }
        movie
    }

    // Frame manipulation
    pub fn add_frame(&mut self, frame: &Frame) {
        self.frames.push(frame.clone());
    }

    // Overide functions
    pub fn override_with_movie(&mut self, movie: &Movie) {
        self.override_with_movie_at_position(0, 0, movie)
    }

    pub fn override_with_movie_at_position(&mut self, x: i32, y: i32, movie: &Movie) {
        let mut i: u32 = 0;
        for frame in self.frames.iter_mut() {
            let _ = frame.override_with_frame_at_position(x, y, &movie.frames[i as usize]);
            i += 1;
            if i >= movie.frames.len() as u32 {
                i = 0;
            }
        }
    }

    pub fn override_with_movie_at_position_in_range(
        &mut self,
        x: i32,
        y: i32,
        movie: &Movie,
        range: std::ops::Range<u32>,
    ) {
        let copied_frame_indexes: Vec<u32> = range.collect();

        let mut i: u32 = 0;
        for (frame_index, frame) in self.frames.iter_mut().enumerate() {
            if copied_frame_indexes.contains(&(frame_index as u32)) {
                let _ = frame.override_with_frame_at_position(x, y, &movie.frames[i as usize]);
                i += 1;
                if i >= movie.frames.len() as u32 {
                    i = 0;
                }
            }
        }
    }

    pub fn play(&self, frames_per_second: u32) {
        for frame in self.frames.iter() {
            print!("\x1B[2J\x1B[1;1H");
            frame.print();
            thread::sleep(time::Duration::from_millis(
                (1000 / frames_per_second).into(),
            ));
        }
    }
}

fn main() {
    
}
