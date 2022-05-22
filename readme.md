# Terminal Image

A simple utility for displaying an image in the terminal with full rgb. [Supported formats.](https://github.com/image-rs/image/blob/master/README.md#supported-image-formats)

If needed, the image will be scaled down by an integer factor to fit inside the terminal, taking the average pixel values.

Uses unicode '▀' to fit two pixels into one character (using fg and bg color).

### Usage
```
cargo build --release
./target/release/terminal-image my-image.png
```
