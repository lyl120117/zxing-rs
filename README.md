# 1. Introduction
zxing-rs is an open-source, multi-format 1D/2D barcode image processing library implemented in Rust, with ports to other languages. 

- [hbar](hbar/) ports to other languages. 
- [hbar-core](hbar-core/) multi-format 1D/2D barcode image processing library implemented in Rust. 
- [ports](ports/) Other languages examples.

# 2. How to use
## 2.1 Run
> cargo run --bin encode

## 2.2 Run with arguments
- Help
> cargo run --bin encode -- --help
- Run with arguments
> cargo run --bin encode -- -b QRCode "Hello"
