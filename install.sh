#!\usr\bin\bash

# Install QEMU Virtual Machine For Testing
apt-get install qemu -y

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Prerequisites 
rustup component add rust-src
rustup component add llvm-tools-preview

# Install Bootimage Tool
cargo install bootimage

# Create New Blank Drive.
qemu-img create drive.img 128M
