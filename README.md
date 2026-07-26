<h1 align=center> Locality and the Costs of Loads and Stores </h1>
<h2 align=center> A CSC 411: Computer Organization Assignment by Zach Breene & C. Wyatt Polasek </h2>
<h4 align=center> Created at the University of Rhode Island, October 2023 </h4>

## Introduction
The purpose of this assignment was to explore cache performance and spatial locality by implementing an image-rotation program called `ppmtrans`. Using the `Array2` data structure from a previous assignment, the project evaluates the performance costs of image rotation by benchmarking different array-access patterns (row-major vs. column-major) to observe the real-world effects of memory loads and stores. 

---

## Implementation + Functions
### locality/array2/src/lib.rs

This directory houses the updated `Array2` library, functioning as the foundational 2D array representation. <br>

&emsp; ***Array Abstraction Method***

* The `Array2` structure is utilized to represent the 2D array of pixels read from a PPM image file.
* It natively supports column-major and row-major mapping alongside column-major or row-major storage.
* The iterator module inside `Array2` provides `iter_row_major` and `iter_col_major` functions, ensuring that pixels are traversed consistently without relying on nested coordinate loops.

### locality/ppmtrans/src/main.rs & transformations.rs

This project directory contains the logic for processing the PPM image and executing the geometric transformations. <br>

&emsp; ***Image Transformation Method***

* The program reads a single PPM image from standard input or a named file, parsing the data into an `Array2` structure.
* A Transformation Module handles the logic for various manipulations, including rotations (0, 90, 180, 270 degrees), flipping (horizontal or vertical), and transposition across the UL-to-LR axis.
* Instead of altering pixel color codes, the transformation algorithms mathematically reorder the pixels—for example, moving a pixel at `(x, y)` to `(y, -x)` for a 90-degree clockwise rotation—while ensuring the output dimensions remain constant.

&emsp; ***Experimental Locality & Benchmarking Method***

* The project includes a theoretical analysis of cache hit rates based on the amount of additions, loads, and stores required per pixel.
* The 90-degree column-major rotation was predicted to have the best cache performance (Rank 1) due to optimal spatial locality, while the 90-degree row-major rotation was predicted to perform the worst (Rank 4) because mapping rows to columns causes strided, cache-unfriendly memory accesses.
* To test these hypotheses, the program utilizes `std::time::Instant` to benchmark solely the time spent computing the geometric rotation of large images (ignoring file I/O times).

---

## How To Run
**IMPORTANT: Ensure you have a working Rust environment and images large enough to exceed the cache size for accurate benchmarking.**

This project workspace is organized within the `locality` parent directory.
* To run the image transformations, navigate to the `locality/ppmtrans` directory.
* Execute the binary via standard input or file paths, passing in the required CLAP command-line arguments for the transformation type and the access mapping. 
* *Example execution:* `djpeg egrets.jpg | target/release/ppmtrans --rotate 90 --row-major > destination.ppm`.
* For unsupported transformation options, the program will reject the input, write an error message to `stderr`, and terminate with a nonzero exit code.

---

## Contribution
* **Partners:** Zach Breene and C. Wyatt Polasek.
* **Implementation Status:** The `ppmtrans` architecture was built with modular transformation and iterator components to ensure mathematical consistency during pixel reordering. The theoretical estimates for loads, stores, and cache misses were fully documented to predict the locality costs prior to benchmarking.
