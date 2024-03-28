# Rust based Operating System

> [!WARNING]\
> Work In Progress!

Following blog series: https://os.phil-opp.com/

#### Create the bootimage

```bash
cargo bootimage
```

#### Booting in QEMU

```bash
qemu-system-x86_64 -drive format=raw,file=target/x86_64-rusty_os/debug/bootimage-rusty_os.bin
```

#### Using the runner

```bash
cargo run
```

##### Track

continue from https://os.phil-opp.com/testing/#testing-the-vga-buffer
