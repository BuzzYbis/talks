# Algorithmic Performance Optimisation: Binary Search Beyond Big-O

This presentation is a talk about optimising algorithm. 
We use binary search as an example to demonstrate optimisation.

The presentation is inside the `presentation.typ` file and the code snippet are in the `code` directory. 

## Build and run

The presentation PDF shall be build using [typst](https://typst.app). 

To build the code snippet: 
- for the rust code, you will only need the stable version of rust
- `config.toml` file will trigger the good SIMD intrinsics alone, you only have to run:
    ```shell
    cargo run --release
    ```
    inside the [./code/rust](./code/rust) directory.
  
Be careful the code will generate approximately 24GB of data in RAM. 
Ensure that you have at least 48GB of RAM to see the true performance of the implementations. 

If you have less than 48GB of RAM, comment some part of the benchmark function and run multiple time to get every number.
If you have less than 24GB of RAM comment the benchmark test for billions od elements. 

If you are on linux, you can run the program 2 times: 
- one time with basic pages
- one time with huge pages

this will allow you to see the difference huge pages make. 
