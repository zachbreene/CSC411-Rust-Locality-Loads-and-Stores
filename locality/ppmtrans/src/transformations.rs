//C. Wyatt Polasek + Zach Breene
//Ppmtrans Transformations

/*For help solving this program, we used the following resources:
GitHub Copilot
https://doc.rust-lang.org/std/vec/struct.Vec.html
https://doc.rust-lang.org/rust-by-example/trait/iter.html
https://doc.rust-lang.org/std/option/enum.Option.html
https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html
https://doc.rust-lang.org/rust-by-example/custom_types/enum.html
https://doc.rust-lang.org/book/ch06-02-match.html
https://doc.rust-lang.org/rust-by-example/flow_control/match.html
https://users.rust-lang.org/t/which-one-should-i-use-panic-vs-unreachable/69401
https://docs.rs/csc411_image/latest/csc411_image/struct.RgbImage.html
CSC411 Notes, PDFs, and Pictures of the Board
TA Office Hours on 10/26/23
*/

use array2::Array2; //Import Array2
use csc411_image::{Rgb, RgbImage}; //Import Rgb and RgbImage from csc411_image
use std::time::Instant; //Benchmarking

/*
Define a public enum to represent the various transformations
Each transformation has a corresponding function in transformations.rs
I used an enum because it felt like a natural way to represent the transformations
I used the following resources to learn about enums:
https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html
https://doc.rust-lang.org/rust-by-example/custom_types/enum.html
*/
#[derive(Debug, Clone)]
pub enum Transformation {
    Rotate90,
    Rotate180,
    Rotate270,
    Rotate0,
    FlipHorizontal,
    FlipVertical,
    Transpose,
}

/*
Defines a public function to perform the transformation on the image
This function takes the image, the transformation, the row-major and column-major flags, and the benchmarking flag as arguments
The match statement is used to determine which transformation to perform
I used match since it was spoken on in class
I used the following resources to learn about match:
https://doc.rust-lang.org/book/ch06-02-match.html
https://doc.rust-lang.org/rust-by-example/flow_control/match.html
Benchmarking code was made following the example showed in the assignment PDF
*/
pub fn perform_transformation(image: &RgbImage, transformation: Transformation, row_major: bool, col_major: bool, benchmark: bool) -> RgbImage {
    //Gets the image dimensions
    let width = image.width as usize;
    let height = image.height as usize;

    //Creates an Array2 from the image pixels
    let image_array = Array2::from_row_major(width, height, image.pixels.clone());

    //Creates an iterator over the image pixels
    //Determines the iteration order based on the command-line options
    let iterator: Vec<_> = if row_major {
        image_array.iter_row_major().collect()
    } else if col_major {
        image_array.iter_col_major().collect()
    } else {
        panic!("Either --row-major or --col-major must be specified.");
    };

    //Determines the new dimensions of the image based on the transformation
    let (new_width, new_height) = match transformation {
        Transformation::Rotate90 | Transformation::Rotate270 | Transformation::Transpose => (height, width),
        _ => (width, height),
    };

    //Creates a new Array2 to store the transformed image
    let mut array = Array2::new(new_width, new_height, Rgb { red: 0, green: 0, blue: 0 });

    //Benchmark: Starting the timer if benchmarking is enabled
    let now = if benchmark { Some(Instant::now()) } else { None };

    //Performs the specified transformation
    match transformation {
        Transformation::Rotate90 => {
            for (x, y, pixel) in iterator {
                array.set(height - 1 - y, x, pixel.clone());
            }
        },
        Transformation::Rotate180 => {
            for (x, y, pixel) in iterator {
                array.set(width - 1 - x, height - 1 - y, pixel.clone());
            }
        },
        Transformation::Rotate270 => {
            for (x, y, pixel) in iterator {
                array.set(y, width - 1 - x, pixel.clone());
            }
        },
        Transformation::Rotate0 => {
            for (x, y, pixel) in iterator {
                array.set(x, y, pixel.clone());
            }
        },
        Transformation::FlipHorizontal => {
            for (x, y, pixel) in iterator {
                array.set(width - 1 - x, y, pixel.clone());
            }
        },
        Transformation::FlipVertical => {
            for (x, y, pixel) in iterator {
                array.set(x, height - 1 - y, pixel.clone());
            }
        },
        Transformation::Transpose => {
            for (x, y, pixel) in iterator {
                array.set(y, x, pixel.clone());
            }
        },
    }

    //If benchmarking is enabled, the elapsed time is printed
    if let Some(now) = now {
        //Benchmark: Stopping the timer
        let elapsed = now.elapsed();
        //Benchmark: Prints elapsed time (ms)
        eprintln!("{:.2?}", elapsed);
    }

    //Constructs and returns a new RgbImage from the transformed Array2
    RgbImage {
        pixels: array.data().clone(),
        width: new_width as u32,
        height: new_height as u32,
        denominator: image.denominator,
    }
}