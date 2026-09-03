use std::io::{self, BufWriter};

use rt::{Canvas, Color};

fn main() -> io::Result<()> {
    const WIDTH: usize = 256;
    const HEIGHT: usize = 192;

    let mut canvas = Canvas::new(WIDTH, HEIGHT);
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let red = x as f64 / (WIDTH - 1) as f64;
            let green = y as f64 / (HEIGHT - 1) as f64;
            canvas.set_pixel(x, y, Color::new(red, green, 0.25));
        }
    }

    let stdout = io::stdout();
    let mut output = BufWriter::new(stdout.lock());
    canvas.write_ppm(&mut output)
}
