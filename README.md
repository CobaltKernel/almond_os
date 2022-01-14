# AlmondOS - A Terrible Kernel

## To-Do
- [ ] - Kernel Level Debugger
  - [x] - Peek Memory
  - [ ] - Assemble Small Routines
  - [x] - Peek Register values
  - [ ] - Write Memory To Disk
  - [x] - Disassemble Memory

- [ ] - Keyboard I/O
  - [ ] - Double ended Queue
  - [x] - Input System
  
- [ ] - Program Loading
  - [ ] - Segment Mapping
  - [ ] - ELF64 Loader
  - [ ] - Flat Binary Loader
  - [ ] - Linking?

- [x] - File System
  - [x] - Contact Vincent Ollivier About Implementing MorosFS As A
          Standalone FileSystem. 
  - [x] - Implement MOROS FS.

- [ ] - MMU API
  - [ ] - Requesting  Pages
  - [ ] - Setting Flags On Pages
  - [ ] - Removing Pages

## Installation
Linux (Recommended)
```bash
git clone https://github.com/CobaltKernel/almond_os.git
sudo ./install.sh
```

## Compiling & Running
Linux 
```bash
./build.sh
```