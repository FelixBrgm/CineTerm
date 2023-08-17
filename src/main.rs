#[allow(unused_imports)]
use std::fs::File;
use std::io::Read;
use std::thread;
use std::time;

/// A struct representing a frame with pixels arranged in a grid.
pub struct Frame {
    /// The height of the frame (number of rows).
    height: usize,
    /// The width of the frame (number of columns).
    width: usize,
    /// A 2D vector containing the pixels of the frame.
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

#[allow(dead_code)]
impl Frame {
    /// Creates a new `Frame` instance with the specified dimensions.
    ///
    /// This function initializes a new `Frame` instance with the given height and width.
    /// Each pixel in the frame is initialized to a space character (' ').
    ///
    /// # Arguments
    ///
    /// * `height` - The height (number of rows) of the frame.
    /// * `width` - The width (number of columns) of the frame.
    ///
    /// # Returns
    ///
    /// Returns a new `Frame` instance with the specified dimensions and all pixels set to spaces.
    pub fn new(height: usize, width: usize) -> Self {
        Frame {
            pixels: vec![vec![' '; width]; height],
            height,
            width,
        }
    }

    /// Creates a new `Frame` instance from a string.
    ///
    /// This function takes a reference to a string and creates a new `Frame` instance
    /// with a single row and a width equal to the length of the input string.
    /// The contents of the string are used to populate the row of pixels in the frame.
    ///
    /// # Arguments
    ///
    /// * `str` - A reference to a string containing the pixel data for the frame.
    ///
    /// # Returns
    ///
    /// Returns a new `Frame` instance with a single row of pixels populated from the input string.
    pub fn from_string(str: &str) -> Self {
        let mut frame = Frame::new(1, str.len());

        frame.overlay_with_string(str);

        frame
    }

    /// Creates a new `Frame` instance from a file.
    ///
    /// This function reads pixel data from a file specified by the `file_path` argument.
    /// The file is expected to contain rows of characters, each representing a row of pixels.
    /// The number of rows and width of the frame are determined by the `height` and `width` arguments.
    ///
    /// # Arguments
    ///
    /// * `height` - The height (number of rows) of the frame.
    /// * `width` - The width (number of columns) of the frame.
    /// * `file_path` - The path to the file containing pixel data.
    ///
    /// # Returns
    ///
    /// Returns a `Result` where `Ok` contains a new `Frame` instance populated with pixel data from the file,
    /// or an `Err` if there was an error reading the file or populating the frame.
    pub fn from_file(height: usize, width: usize, file_path: &str) -> Result<Self, std::io::Error> {
        let mut file = File::open(file_path)?;
        let mut file_contents = String::new();
        file.read_to_string(&mut file_contents)?;

        let lines: Vec<String> = file_contents.lines().map(String::from).collect();

        let mut frame = Frame::new(height, width);

        for (index, line) in lines.iter().enumerate() {
            frame.overlay_with_string_at_position(0, index as i32, line);
        }

        Ok(frame)
    }

    /// overlays the pixel data of the current frame with the pixel data from another frame.
    ///
    /// This function takes a reference to another `Frame` instance and replaces the pixel data in the current frame
    /// with the pixel data from the provided frame. The operation starts at the top-left corner of the current frame.
    ///
    /// # Arguments
    ///
    /// * `frame` - A reference to another `Frame` instance whose pixel data will overlay the current frame's data.
    pub fn overlay_with_frame(&mut self, frame: &Frame) {
        self.overlay_with_frame_at_position(0, 0, frame);
    }

    /// overlays a portion of the current frame with pixel data from another frame.
    ///
    /// This function takes a reference to another `Frame` instance and replaces a portion of the pixel data
    /// in the current frame with the corresponding portion from the provided frame. The starting position
    /// for the overlay is specified by the `x` and `y` parameters.
    ///
    /// If the starting position (x, y) is outside the bounds of the current frame or the provided frame,
    /// the function will skip those pixels.
    ///
    /// # Arguments
    ///
    /// * `x` - The X-coordinate of the starting position for the overlay.
    /// * `y` - The Y-coordinate of the starting position for the overlay.
    /// * `frame` - A reference to another `Frame` instance whose pixel data will overlay a portion of the current frame's data.
    pub fn overlay_with_frame_at_position(&mut self, x: i32, y: i32, frame: &Frame) {
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

    /// overlays the pixel data of the current frame with characters from a string.
    ///
    /// This function takes a reference to a string and replaces a portion of the pixel data in the current frame
    /// with characters from the input string. The operation starts at the top-left corner of the current frame.
    ///
    /// # Arguments
    ///
    /// * `str` - A reference to a string containing the new pixel data for the frame.
    pub fn overlay_with_string(&mut self, str: &str) {
        self.overlay_with_string_at_position(0, 0, str);
    }

    /// overlays a portion of the current frame with characters from a string at a specific position.
    ///
    /// This function takes a reference to a string and replaces a portion of the pixel data in the current frame
    /// with characters from the input string. The starting position for the overlay is specified by the `x` and `y` parameters.
    ///
    /// If the starting position is outside the width bounds of the frame, the function will only overlay the pixels
    /// that fit within the width of the frame.
    ///
    /// # Arguments
    ///
    /// * `x` - The X-coordinate of the starting position for the overlay.
    /// * `y` - The Y-coordinate of the starting position for the overlay.
    pub fn overlay_with_string_at_position(&mut self, x: i32, y: i32, str: &str) {
        for (c_index, c) in str.as_bytes().iter().enumerate() {
            if c_index < self.width {
                self.pixels[0][c_index] = *c as char;
            }
        }
    }

    /// Gets the width of the current frame.
    ///
    /// This function returns the width (number of columns) of the current frame.
    ///
    /// # Returns
    ///
    /// Returns the width of the frame.
    fn get_width(&self) -> usize {
        self.width
    }

    /// Gets the height of the current frame.
    ///
    /// This function returns the height (number of rows) of the current frame.
    ///
    /// # Returns
    ///
    /// Returns the height of the frame.
    fn get_height(&self) -> usize {
        self.height
    }

    /// Prints the contents of the current frame.
    ///
    /// This function prints the pixel data of the current frame to the console, row by row.
    pub fn print(&self) {
        for line in &self.pixels {
            for c in line {
                print!("{}", *c);
            }
            print!("\n");
        }
    }
}

/// A struct representing a movie composed of frames.
pub struct Movie {
    /// The width of each frame in the movie.
    width: usize,
    /// The height of each frame in the movie.
    height: usize,
    /// The collection of frames that make up the movie.
    frames: Vec<Frame>,
}

#[allow(dead_code)]
impl Movie {
    /// Creates a new empty `Movie` instance with the specified dimensions.
    ///
    /// This function initializes a new `Movie` instance with the given height and width.
    ///
    /// # Arguments
    ///
    /// * `height` - The height (number of rows) of each frame in the movie.
    /// * `width` - The width (number of columns) of each frame in the movie.
    ///
    /// # Returns
    ///
    /// Returns a new empty `Movie` instance with the specified dimensions.
    pub fn new(height: usize, width: usize) -> Self {
        Movie {
            height,
            width,
            frames: vec![],
        }
    }

    /// Creates a new `Movie` instance with blank frames.
    ///
    /// This function initializes a new `Movie` instance with the given height and width, and populates it
    /// with a specified number of blank frames (frames with all pixels set to spaces).
    ///
    /// # Arguments
    ///
    /// * `height` - The height (number of rows) of each frame in the movie.
    /// * `width` - The width (number of columns) of each frame in the movie.
    /// * `frame_amount` - The number of blank frames to add to the movie.
    ///
    /// # Returns
    ///
    /// Returns a new `Movie` instance with blank frames and the specified dimensions.
    pub fn new_blank(height: usize, width: usize, frame_amount: usize) -> Self {
        let mut movie = Movie::new(height, width);
        let blank_frame = Frame::new(height, width);
        for _i in 0..frame_amount {
            movie.add_frame(&blank_frame);
        }
        movie
    }

    // Frame manipulation
    /// Adds a frame to the movie.
    ///
    /// This function adds a new frame to the collection of frames in the movie.
    ///
    /// # Arguments
    ///
    /// * `frame` - A reference to the frame to be added to the movie.
    pub fn add_frame(&mut self, frame: &Frame) {
        self.frames.push(frame.clone());
    }

    /// Overlays the movie's frames with frames from another movie.
    ///
    /// This function takes a reference to another `Movie` instance and overlays the movie's frames with
    /// frames from the provided movie. The operation starts at the top-left corner of the movie.
    ///
    /// # Arguments
    ///
    /// * `movie` - A reference to another `Movie` instance whose frames will overlay the current movie's frames.
    ///
    /// # Example
    /// ```
    /// "XXX".overlay_with_movie("O") == "OXX"
    /// ```
    pub fn overlay_with_movie(&mut self, movie: &Movie) {
        self.overlay_with_movie_at_position(0, 0, movie)
    }

    /// Overlay the frames of the current movie with the frames of another movie at the specified position (x, y).
    ///
    /// This method overlays the frames of the provided `movie` onto the frames of the current movie
    /// starting from the given position (x, y). The overlay is performed sequentially for each frame
    /// in the current movie, using frames from the provided `movie` in a looping manner.
    ///
    /// # Arguments
    ///
    /// * `x` - The x-coordinate of the position where the overlay will start.
    /// * `y` - The y-coordinate of the position where the overlay will start.
    /// * `movie` - A reference to the `Movie` whose frames will be overlaid onto the current movie's frames.
    /// # Example
    /// ```
    /// "XXX".overlay_with_movie_at_position(0,1," O") == "X O"
    /// ```
    pub fn overlay_with_movie_at_position(&mut self, x: i32, y: i32, movie: &Movie) {
        let mut i: u32 = 0;
        for frame in self.frames.iter_mut() {
            let _ = frame.overlay_with_frame_at_position(x, y, &movie.frames[i as usize]);
            i += 1;
            if i >= movie.frames.len() as u32 {
                i = 0;
            }
        }
    }

    /// overlays specific frames in the movie with frames from another movie at positions within a range.
    ///
    /// This function takes a reference to another `Movie` instance and overlays specific frames in the movie's collection
    /// with frames from the provided movie. The overlay positions are determined by the `range` parameter.
    ///
    /// # Arguments
    ///
    /// * `x` - The X-coordinate of the starting position for the overlay.
    /// * `y` - The Y-coordinate of the starting position for the overlay.
    /// * `movie` - A reference to another `Movie` instance whose frames will overlay the current movie's frames.
    /// * `range` - A range specifying the frame indexes to be overridden within the movie.
    pub fn overlay_with_movie_at_position_in_range(
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
                let _ = frame.overlay_with_frame_at_position(x, y, &movie.frames[i as usize]);
                i += 1;
                if i >= movie.frames.len() as u32 {
                    i = 0;
                }
            }
        }
    }

    /// Plays the movie by printing its frames in sequence.
    ///
    /// This function displays the frames of the movie on the console, simulating animation.
    /// The console screen is cleared before printing each frame to achieve a smooth animation effect.
    ///
    /// # Arguments
    ///
    /// * `frames_per_second` - The number of frames to be displayed per second.
    pub fn play(&self, frames_per_second: u32) {
        for frame in self.frames.iter() {
            print!("\x1B[2J\x1B[1;1H");
            frame.print();
            thread::sleep(time::Duration::from_millis(
                (1000 / frames_per_second).into(),
            ));
        }
    }

    /// Gets the width of the frames in the movie.
    ///
    /// This function returns the width (number of columns) of the frames in the movie.
    ///
    /// # Returns
    ///
    /// Returns the width of the frames in the movie.
    fn get_width(&self) -> usize {
        self.width
    }

    /// Gets the height of the frames in the movie.
    ///
    /// This function returns the height (number of rows) of the frames in the movie.
    ///
    /// # Returns
    ///
    /// Returns the height of the frames in the movie.
    fn get_height(&self) -> usize {
        self.height
    }
}
