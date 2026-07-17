# Rust OS

A bare-metal Rust OS kernel utilizing the Limine bootloader protocol v8.x.

This repository demonstrates how to set up a minimal `x86_64` rust kernel that uses `limine` `0.6.5` to retrieve a framebuffer, draw a built-in bitmap font (`font8x8`) to print "Hello World!" on the screen, and output "Hello World!" to the serial port as a fallback.

## Building and Running

1. **Build and Run**
   ```bash
   cargo run
   ```
   The `runner.sh` script will automatically:
   - Build the Rust kernel via Cargo
   - Clone the Limine binary release
   - Build a bootable ISO with `xorriso` and deploy `limine`
   - Boot the ISO in QEMU

## Design Notes

- The Limine crate version `0.6.5` relies on raw protocol structures such as `BaseRevision`, `RequestsStartMarker`, `RequestsEndMarker`, and the standard `FramebufferRequest`.
- Built-in terminal requests were dropped from Limine 5.x+ and therefore they do not exist in the `limine` `0.6.5` crate.
- We utilize `font8x8` to iterate over character glyphs and plot the pixels directly to the `Framebuffer` slice obtained via Limine.
- Serial port output is written to `0x3F8` using inline `outb` assembly.
