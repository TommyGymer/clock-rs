# clock-rs

A simple analog clock written in Rust using the bevy engine

## Example

![image of basic clock-rs](https://github.com/user-attachments/assets/8e6b50c5-acf3-4cf6-9bdd-3eb0a69f055b)

## Building

May need to [install bevy dependencies](https://bevyengine.org/learn/quick-start/getting-started/setup/) before compiling.
It may also be neccessary to use the below modified cargo command due to the large number of dependencies that bevy uses:
```bash
RUST_MIN_STACK=<16777216 or some equally large number of bytes> cargo build --release
```
