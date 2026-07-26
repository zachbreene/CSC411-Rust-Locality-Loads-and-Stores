//C. Wyatt Polasek + Zach Breene
//Ppmtrans Main

/*For help solving this program, we used the following resources:
GitHub Copilot
https://doc.rust-lang.org/std/vec/struct.Vec.html
https://doc.rust-lang.org/rust-by-example/trait/iter.html
https://doc.rust-lang.org/book/ch07-02-defining-modules-to-control-scope-and-privacy.html
https://docs.rs/clap/latest/clap/
https://docs.rs/clap/latest/clap/_derive/
https://docs.rs/clap/latest/clap/struct.ArgMatches.html#method.parse
https://users.rust-lang.org/t/which-one-should-i-use-panic-vs-unreachable/69401
https://doc.rust-lang.org/book/ch06-02-match.html
https://doc.rust-lang.org/rust-by-example/flow_control/match.html
https://docs.rs/csc411_image/latest/csc411_image/struct.RgbImage.html
https://docs.rs/csc411_image/latest/csc411_image/trait.Read.html
https://docs.rs/csc411_image/latest/csc411_image/trait.Write.html
CSC411 Notes, PDFs, and Pictures of the Board
TA Office Hours on 10/26/23
*/

//Import array2
extern crate array2;

//Import tranformations module
mod transformations; 
//Import enum and function from the transformations module
use transformations::{Transformation, perform_transformation};

use csc411_image::{Read, Write, RgbImage};
//Parser trait is used to parse command-line arguments
use clap::Parser;
//Error trait for returning errors
use std::error::Error;

/*
Struct to represent the command-line arguments
The clap macro is used to parse the command-line arguments
I used the clap macro since it was recommended in the assignment PDF
I used the following resources to learn about the clap macro:
https://docs.rs/clap/latest/clap/
https://docs.rs/clap/latest/clap/_derive/
*/ 
#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Args {
    //Input file
    input_file: Option<String>,
    //Transformation options
    #[clap(long = "rotate", possible_values = &["90", "180", "270", "0"], takes_value = true)]
    rotate: Option<String>,
    #[clap(long = "flip", possible_values = &["horizontal", "vertical"], takes_value = true)]
    flip: Option<String>,
    #[clap(long = "transpose")]
    transpose: bool,
    //Row-major or Column-major iterator options
    #[clap(long = "row-major")]
    row_major: bool,
    #[clap(long = "col-major")]
    col_major: bool,
    //Benchmarking option
    #[clap(long = "benchmark")]
    benchmark: bool,
    //Output file
    output_file: Option<String>,
}

/*
Main function
This function parses the command-line arguments and performs the transformation on the image
Result<(), Box<dyn Error>> is used to return an error if the program fails
unreachable!() is used to indicate that the program should never reach that point
Args::parse() is used to parse the command-line arguments
I used match since it was spoken on in class
I used the following resources to learn about match:
https://doc.rust-lang.org/book/ch06-02-match.html
https://doc.rust-lang.org/rust-by-example/flow_control/match.html
 */
fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    
    //Determine the transformation based on the command-line options
    let transformation = if let Some(angle) = args.rotate {
        match angle.as_str() {
            "90" => Transformation::Rotate90,
            "180" => Transformation::Rotate180,
            "270" => Transformation::Rotate270,
            "0" => Transformation::Rotate0,
            _ => unreachable!(), //Unreachable is used here to indicate that the program should never reach that point
        }
    } else if let Some(direction) = args.flip {
        match direction.as_str() {
            "horizontal" => Transformation::FlipHorizontal,
            "vertical" => Transformation::FlipVertical,
            _ => unreachable!(), //Unreachable is used here to indicate that the program should never reach that point
        }
    } else if args.transpose {
        Transformation::Transpose
    } else {
        //If no transformation is specified, print an error and the program exits
        eprintln!("Error: You must specify a transformation.");
        std::process::exit(1);
    };
    
    //Read the image from the input file
    let image = RgbImage::read(args.input_file.as_deref())?;
    //Perform the transformation on the image
    let transformed_image = perform_transformation(&image, transformation, args.row_major, args.col_major, args.benchmark);
    //Write the transformed image to the output file
    transformed_image.write(args.output_file.as_deref())?;
    //Returns Ok if the program succeeds
    Ok(())
}