# Ray Tracing
## How to build
Please prepare rust toolchain and proceed the following command in the project root.
THe code should be quite protable. Though it is only tested on MacOS, it should work fine on Linux and Windows as well.

Use the following command to build the project:
```bash
cargo build
```
Then the executable file is `target/debug/raytracing` file
## How to use
Please provide 2 arguments, first is the input text file path, second is the output image path.
For example,
```bash
./target/debug/raytracing hw1_input.txt output.ppm
```

## Current State
Only intersection test is performed and there's no real ray tracing now.
It will be implemented in hw2 branch.