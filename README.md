## Toolchain

This project uses a nightly toolchain, `nightly-2019-10-14`, because the code running on CKB-VM.

```
rustup target add riscv32imac-unknown-none-elf
```

## Building a bot

```
cargo build -p example_bot --target=riscv32imac-unknown-none-elf
```

## Running a bot

```
cargo run -p ff_local -- target/riscv32imac-unknown-none-elf/debug/ff-example-bot target/riscv32imac-unknown-none-elf/debug/ff-example-bot
```
