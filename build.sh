#!/usr/bin/bash
nasm disk/hello.asm -f elf64
tar -cf disk.img disk
cargo run