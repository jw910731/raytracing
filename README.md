# Ray Tracing

## How to build

Please prepare rust toolchain and proceed the following command in the project root.
THe code should be quite protable. Though it is only tested on MacOS, it should work fine on Linux and Windows as well.

Use the following command to build the project:

```bash
cargo build -r
```

Then the executable file is `target/debug/raytracing` file

## How to use

Please provide 2 arguments, first is the input text file path, second is the output image path.
For example:

```bash
./target/release/raytracing hw2_input.txt output.ppm
```

Third argument is optional, it means antialiasing level, any integer is valid, but power of 4 is recommended.
In the following example, the output image is rendered in 4 samples per pixel:

```bash
./target/release/raytracing hw2_input.txt output.ppm 4
```

## Current State

Phong lightning model and path tracing based reflection is implemented.

Everything runs on CPU in parallel.
