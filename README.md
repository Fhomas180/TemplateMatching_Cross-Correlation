      BENCHMARK RESULTS
Sequential:  1.518s
Parallel:    0.161s
Speedup:     9.45x
Efficiency:  29.53%
Accuracy:    82.34%

--- Two Parallel Approaches ---
Rayon par_iter:    0.161s
std::thread + Arc: 0.350s
Rayon speedup over std::thread: 2.18x

[dependencies]
mnist = "0.5"
textplots = "0.8"
rayon = "1.7"
ndarray = "0.15"

rustFinalProject/
├── Cargo.toml
├── Cargo.lock
├── README.md
├── src/
│   ├── main.rs         # Entry point, timing, benchmarking
│   ├── lib.rs          # Public module declarations
│   ├── algorithm.rs    # normalize_image, dot_product, find_best_match
│   ├── sequential.rs   # build_templates, classify_sequential
│   ├── parallel.rs     # classify_parallel (Rayon), classify_parallel_threads (std::thread)
│   └── benchmarks.rs   # BenchmarkResults, print_results, plot_speedup_by_threads
├── tests/
│   └── tests.rs        # Unit tests for algorithm functions
├── benchmarks/
│   └── results.txt     # Benchmark results from both systems
└── data/
    └── (see Getting Started)
###### References 

- https://www.kaggle.com/datasets/hojjatk/mnist-dataset/data    <------- this is for MNIST Dataset
- https://docs.rs/rayon/latest/rayon/   <----------------- Rayon Documentation
- https://docs.rs/ndarray/latest/ndarray/  <--------------------------- ndarray documentation
- Tempatle and Cross Correlation Documentation -
- https://vipin-sharma.medium.com/image-template-matching-using-cross-correlation-2f2b8e59f254
- https://www.ipb.uni-bonn.de/html/teaching/photo12-2021/2021-pho1-09-matching-cc.pptx.pdf
- https://blog.roboflow.com/template-matching/
- https://doc.rust-lang.org/std/time/struct.Instant.html <--------- Rust std::time::instant code
# Template Matching & Cross Correlation WITH MNIST HandWritten Digits.

**Made By Thomas Flavin, Course: CSI-380 Emerging Languages Final Project**

## Project Description

**This project focuses on Template Matching / Cross Correlation algorithm to get handwritten digits from the MNIST dataset. It compares a sequential single-threaded implementation against a parallel multi-threaded implementation  using Rayon. Its performance is measured by using the execution time, speedup, efficiency, and classification accuracy**




### Prerequisites

**Rust Version** - rustc 1.91.1 or later
**RAM:** 16GB recommended
**CPU:** Multi-core processor recommended
**OS:** Windows, macOS,


#### Getting Started

1. Extract the .zip file
2. Make a data file folder for the MNIST file
3. Download the MNIST file from this website called KAGGLE [this link](https://www.kaggle.com/datasets/hojjatk/mnist-dataset/data)
4. After Mnist file have downloaded you should see all four folders and four files make sure you only get these files
 - train-images-idx3-ubyte
- train-labels-idx1-ubyte
- t10k-images-idx3-ubyte
- t10k-labels-idx1-ubyte
5. Put that data inside of the data/ which should be like this 
rustFinalProject/
└── data/
├── train-images-idx3-ubyte
├── train-labels-idx1-ubyte
├── t10k-images-idx3-ubyte
└── t10k-labels-idx1-ubyte

##### Running the program
1. to run the full program make sure you use cargo run. it should hopefully look like this btw your data would look complete different if you have different RAM or OS
2.
- Finished `dev` profile [unoptimized + debuginfo] target(s) in 5.66s
- Running `target\debug\rustFinalProject.exe`
- Building templates...
- Templates built!
- Running sequential classification...
- Sequential done in 1.984s
- Running parallel classification...
- Parallel done in 0.170s
- Running the thread scaling test...





Threads:  1 | Time: 1.900s | Speedup: 1.04x
Threads:  2 | Time: 1.066s | Speedup: 1.86x
Threads:  4 | Time: 0.639s | Speedup: 3.10x
Threads:  8 | Time: 0.380s | Speedup: 5.23x
Threads: 16 | Time: 0.240s | Speedup: 8.26x
Threads: 24 | Time: 0.192s | Speedup: 10.35x
 BenchMark Results
Sequential:  1.984s
Parallel:    0.170s
Speedup:     11.66x
Efficiency:  36.44%
Accuracy:    82.34%

Execution Time by Thread Count:
(x = threads, y = seconds)
⡁⠀⢸⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀ 2.0
⠄⠀⠀⢇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠂⠀⠀⠘⡄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⡁⠀⠀⠀⠱⣀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠄⠀⠀⠀⠀⠀⠑⢄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠂⠀⠀⠀⠀⠀⠀⠀⠑⢄⣀⣀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⡁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠉⠉⠑⠒⠒⠤⠤⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠉⠉⠉⠉⠉⠉⠉⠉⠉⠑⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠢⠀⠀⠀ 0.2
0.0                                                     25.0


[dependencies]
image = "0.24"
imageproc = "0.23"
mnist = "0.5"
textplots = "0.8"
rayon = "1.7"
ndarray = "0.15"


###### References 

- https://www.kaggle.com/datasets/hojjatk/mnist-dataset/data    <------- this is for MNIST Dataset
- https://docs.rs/rayon/latest/rayon/   <----------------- Rayon Documentation
- https://docs.rs/ndarray/latest/ndarray/  <--------------------------- ndarray documentation
- Tempatle and Cross Correlation Documentation -
- https://vipin-sharma.medium.com/image-template-matching-using-cross-correlation-2f2b8e59f254
- https://www.ipb.uni-bonn.de/html/teaching/photo12-2021/2021-pho1-09-matching-cc.pptx.pdf
- https://blog.roboflow.com/template-matching/
- https://doc.rust-lang.org/std/time/struct.Instant.html <--------- Rust std::time::instant code
