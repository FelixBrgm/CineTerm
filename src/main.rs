use cine_term::{Color, Frame, Movie};

fn main() {
    let mut f = Frame::new(100, 100);

    f.fill('r');

    let mut movie = Movie::new(100, 100);
    for i in 0..100 {
        if i % 2 == 0 {
            f.set_color(&Color::Blue);
        } else {
            f.set_color(&Color::Red);
        }
        movie.add_frame(&f);
    }

   //  movie.play(20);

    let m = Movie::new(100, 100);

    movie.overlay_with_movie(&m);
}
