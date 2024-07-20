### eBPF Network Blocking POC in Rust
This POC demonstrates blocking network traffic to a specific destination port using eBPF in Rust.

### Features:

Blocks traffic to a user-specified port.
Logs blocked packets with source and destination information.
Command-line arguments for port configuration.
### Requirements:

```
Rust compiler (>= 1.56)
cargo package manager
sudo privileges (for attaching eBPF program)
aya_ebpf library (cargo install aya_ebpf)
```
Steps to run the program:
# Take the build of ebpf program
cargo xtask build-ebpf
# command to see the output
llvm-objdump -S target/bpfel-unknown-none/debug/network-block
# run the program
cargo xtask run -- -h
# see logs(run in a new terminal)
RUST_LOG=info cargo xtask run -- --iface wlp58s0
# After stoppping using Ctrl+C checking the program get detached
sudo bpftool prog list
