use std::fs::File;
use std::str::FromStr;

use image::{png::PNGEncoder, ColorType};
use num::{complex::Complex64, Complex, Zero};

/// This number is the threshold for divergence
const BIG: f64 = 4.0;

type Point<T> = (T, T);
type Coordinate = Point<usize>;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 5 {
        eprintln!("Usage: {} FILENAME PIXELS UPPERLIEFT LOWERRIGHT", args[0]);
        eprintln!(
            "Example: {} mandelbrot.png 1000x750 -1.20,0.35 -1.0,0.20",
            args[0]
        );
    }

    let filename = &args[1];
    let bounds = parse_pair::<usize>(&args[2], 'x').expect("Could not parse input");
    let upper_left = parse_complex(&args[3], ',').expect("Could not parse input");
    let lower_right = parse_complex(&args[4], ',').expect("Could not parse input");

    let mut pixels = vec![0; bounds.0 * bounds.1];

    if std::env::var("SINGLE").is_ok() {
        render(&mut pixels, bounds, upper_left, lower_right);
    } else {
        let threads = 8;
        println!("Using {} threads", threads);
        let rows_per_band = bounds.1 / threads + 1;
        {
            let bands: Vec<&mut [u8]> = pixels.chunks_mut(rows_per_band * bounds.0).collect();
            crossbeam::scope(|scope| {
                for (i, band) in bands.into_iter().enumerate() {
                    let top = rows_per_band * i;
                    let height = band.len() / bounds.0;
                    let band_bounds = (bounds.0, height);
                    let band_upper_left = pixel_to_point(bounds, (0, top), upper_left, lower_right);
                    let band_lower_right =
                        pixel_to_point(bounds, (bounds.0, top + height), upper_left, lower_right);
                    scope.spawn(move |_| {
                        render(band, band_bounds, band_upper_left, band_lower_right);
                    });
                }
            })
            .unwrap();
        };
    }

    write_image(&filename, &pixels, bounds).expect("could not write to file");
}

/// Returns the number of iterations for number to diverge, returns None if convergent
fn escape(c: Complex64, limit: usize) -> Option<usize> {
    let mut z = Complex64::zero();

    for i in 1..limit {
        if z.norm_sqr() > BIG {
            return Some(i);
        };
        z = z * z + c;
    }

    None
}

fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<Point<T>> {
    match s.find(separator) {
        None => None,
        Some(index) => match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {
            (Ok(left), Ok(right)) => return Some((left, right)),
            _ => return None,
        },
    }
}

fn parse_complex(s: &str, separator: char) -> Option<Complex64> {
    let (re, im) = parse_pair::<f64>(s, separator)?;
    Some(Complex { re, im })
}

#[test]
fn test_parse_pair() {
    assert_eq!(parse_pair::<usize>("400x321", 'x'), Some((400, 321)));
    assert_eq!(parse_pair::<i32>("400,321", 'x'), None);
}

fn pixel_to_point(
    bounds: Coordinate,
    pixel: Coordinate,
    uper_left: Complex64,
    lower_right: Complex64,
) -> Complex64 {
    let (width, height) = (lower_right.re - uper_left.re, uper_left.im - lower_right.im);
    Complex64 {
        re: uper_left.re + pixel.0 as f64 * width / bounds.0 as f64,
        im: uper_left.im - pixel.1 as f64 * height / bounds.1 as f64,
    }
}
#[test]
fn test_pixel_to_point() {
    assert_eq!(
        pixel_to_point(
            (100, 200),
            (25, 175),
            Complex { re: -1.0, im: 1.0 },
            Complex { re: 1.0, im: -1.0 }
        ),
        Complex {
            re: -0.5,
            im: -0.75
        }
    );
}

/// Render a rectangle of the mandelbrot set into a buffer of pixels
fn render(pixels: &mut [u8], bounds: Coordinate, upper_left: Complex64, lower_right: Complex64) {
    assert!(pixels.len() == bounds.0 * bounds.1);
    for row in 0..bounds.1 {
        for column in 0..bounds.0 {
            let point = pixel_to_point(bounds, (column, row), upper_left, lower_right);
            pixels[row * bounds.0 + column] = match escape(point, 255) {
                None => 0,
                Some(count) => 255 - count as u8,
            };
        }
    }
}

fn write_image(filename: &str, pixels: &[u8], bounds: Coordinate) -> Result<(), std::io::Error> {
    let output = File::create(filename)?;

    let encoder = PNGEncoder::new(output);
    encoder.encode(pixels, bounds.0 as u32, bounds.1 as u32, ColorType::Gray(8))?;

    Ok(())
}
