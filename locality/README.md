> C. Wyatt Polasek & Zach Breene <br>
> Assignment 3 - ppmtrans <br>
> README.md


> ### 1. Acknowledgments

- https://doc.rust-lang.org/std/vec/struct.Vec.html
- https://doc.rust-lang.org/rust-by-example/trait/iter.html
- https://doc.rust-lang.org/std/option/enum.Option.html
- https://stackoverflow.com/questions/32304595/whats-the-difference-between-self-and-self
- https://doc.rust-lang.org/reference/expressions/range-expr.html#:~:text=Expression%20RangeFullExpr%20%3A%20,7
- https://stackoverflow.com/questions/27175685/how-to-allocate-space-for-a-vect-in-rust
- https://doc.rust-lang.org/book/ch07-02-defining-modules-to-control-scope-and-privacy.html
- https://docs.rs/clap/latest/clap/
- https://docs.rs/clap/latest/clap/_derive/
- https://docs.rs/clap/latest/clap/struct.ArgMatches.html#method.parse
- https://users.rust-lang.org/t/which-one-should-i-use-panic-vs-unreachable/69401
- https://doc.rust-lang.org/book/ch06-02-match.html
- https://doc.rust-lang.org/rust-by-example/flow_control/match.html
- https://docs.rs/csc411_image/latest/csc411_image/struct.RgbImage.html
- https://docs.rs/csc411_image/latest/csc411_image/trait.Read.html
- https://docs.rs/csc411_image/latest/csc411_image/trait.Write.html
- https://doc.rust-lang.org/std/option/enum.Option.html
- https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html
- https://doc.rust-lang.org/rust-by-example/custom_types/enum.html
- TA's
- Github Copilot

> ### 2. Correctly Implemented Options

We have completed all parts of the assignment listed below:

- Trandsformation Types:
    - --rotate (degree)
        - 0 Degrees
        - 90 Degrees
        - 180 Degrees
        - 270 Degrees
    - --flip (axis)
        - Horizontal Axis
        - Vertical Axis
    - --transpose
        - UL-to-LR Axis

    - Iterator Options:
        - --row-major
        - --col-major

    - Speed Benchmark Option:
        - --benchmark

There are no options that have not been implemented correctly.


> ### 3. Architecture of Solutions

The ppmtrans program is structured to parse the input, determine the required transformation, and then call the appropriate function(s) from the Transformation Module. The Iterator Module is first called to move through the ppm image file and read the data into a 2D array. Then the Transformation Module selects the function corresponding to the chosen transformation type. Each transformation function holds the logic that is applied to each pixel in the image. The Transformation module then utilizes the Iterator Module to traverse the array where the function is applied to each pixel. After the transformation is applied, the transformed pixel is sent to a new 2D array.


> ### 4. Part C: Measured Speeds

Using image of the Chamaeleon-I Molecular Cloud (126 MB)

|     	    | **row-major** | **col-major** |
|-----	    |:-----------:	|:-----------:	|
| **0**     | 2.29 s       	| 4.47 s    	|
| **90**    | 3.37 s    	| 3.51 s    	|
| **180**   | 2.23 s    	| 4.55 s    	|
| **270**   | 3.31 s        | 3.92 s    	|

These results exemplify how important it is for a program to utilize spatial locality when accessing memory. When the memory access pattern is the same as the memory layout, the program can perform much faster. This can be seen clearly in the results when looking at the 0 and 180-degree rotations using row-major access.
<br><br>
Since memory in Rust is stored in row-major order, the program can utilize spatial locality within the cache and perform much faster. When looking at the same rotations using col-major, the program runs into more cache misses that result in a significantly slower performance.


> ### 5. Part D: Better Memory Layout

In the row-major and col-major layouts we used, rotating a rectangular image 90 degrees causes column-wise access that is more likely to result in cache misses. A memory layout that performs better than this could be a layout that breaks an image into smaller blocks. Each of these blocks would be accessed using row-major and sized to accommodate the cache size of the system being used. Since these blocks should match the size of the cache, data accessed inside the block is much less likely to run into misses. This block layout also utilizes spatial locality since adjacent blocks would be stored close to each other in the memory. When it comes to the performance of 90-degree rotations specifically, each block can be accessed individually and easily rearranged rather than accessing an entire row or column one by one.

> ### 6. Approximate Time Taken

Approximately 22-24 hours



Usage
------

> Chamaeleon-I Image (Largest) (n*10^3 ms)
- ./target/release/ppmtrans.exe ./chamaeleon.ppm > ./CH_row_90.ppm --rotate 90 --row-major
- ./target/release/ppmtrans.exe ./chamaeleon.ppm > ./CH_col_90.ppm --rotate 90 --col-major

> Andromeda Galaxy Image (Large) (n*10^2 ms)
- ./target/release/ppmtrans.exe ./andromeda.ppm > ./AND_row_90.ppm --rotate 90 --row-major
- ./target/release/ppmtrans.exe ./andromeda.ppm > ./AND_col_90.ppm --rotate 90 --col-major

> Donkey Kong Image (Small) (n*10 ms)
- ./target/release/ppmtrans.exe ./DonkeyKong.ppm > ./DK_row_90.ppm --rotate 90 --row-major
- ./target/release/ppmtrans.exe ./DonkeyKong.ppm > ./DK_col_90.ppm --rotate 90 --col-major
