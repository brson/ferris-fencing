# Ferris Fencing

> An eternal duel between programmable crabs with swords.

[Ferris Fencing][fforg] is a live tournament in which player-programmed bots combat each
other on a [RISC-V] virtual machine.

It is a showcase of [CKB-VM], a simple implementation of the RISC-V instruction
set, written in the [Rust] programing language.

[www.ferrisfencing.org][fforg]

[fforg]: http://www.ferrisfencing.org
[RISC-V]: https://www.riscv.org
[CKB-VM]: https://github.com/nervosnetwork/ckb-vm
[Rust]: https://www.rust-lang.org


## Status

Ferris Fencing is in early development. It does not yet run in the cloud, and
the final rules are not yet determined.

For now, it is possible to write bots in Rust, compile them to RISC-V, and run
them locally with the Ferris Fencing runtime.

Soon, players will be able to upload their bots to the Ferris Fencing server
to challenge others' bots.


## The Rules

See [www.ferrisfencing.org][fforg] for the game rules.


## Building and running

For simplicity, we suggest building off the code in the brson/ferris-fencing
workspace, which contains the runtime, example bots, command line tools, and the
web server.

This project uses a nightly toolchain, because bots are running on (virtual)
bare-metal RISC-V, and that requires some nightly features. The exact toolchain
is listed in the `rust-toolchain` file in the repo, and will be used
automatically.

For building bots this project requires the `riscv32-imac-unknown-none-elf`
compiler target.

The following commands will set you up:

```
git clone https://github.com/brson/ferris-fencing.git
cd ferris-fencing
rustup target add riscv32imac-unknown-none-elf
```

The repository is a `cargo` workspace that contains the following projects, in
the `src` directory, each of which can be build or run with the `cargo` `-p`
flag:

- `ckb_vm_glue` - A basic bot runtime library containing the boilerplate
  necessary to run `main`. Think of it as `std` for the Ferris Fencing platform.
- `ckb_vm_syscall` - Assembly-language trampolines for calling RISC-V syscalls.
- `example_bot` - A working Ferris Fencing bot.
- `ff_local` - The CLI for running a local match between two bots.
- `ff_rt` - The Ferris Fencing platform runtime. The game logic.
- `ff_web` - The Ferris Fencing web API.
- `ff_web_common` - Support library for the website.
- `ff_web_json` - Runs a match and emits json.

It also contains two `demo_*` projects. These were used in the [Rust.Tokyo 2019]
talk for which Ferris Fencing was made. On their own they are relatively
uninteresting.

[Rust.Tokyo 2019]: https://rust.tokyo

Note that the projects in this workspace are a mixture of standard desktop
projects and RISC-V-specific projects. Because of this, at the moment, running
`cargo` with the `--all` flag will fail. By default `cargo build` and
`cargo run` will build `ff_local`.


### Building the example bot

```
cargo build -p example_bot --target=riscv32imac-unknown-none-elf
```

This will put a binary in

> target/riscv32imac-unknown-none-elf/debug/ff-example-bot


### Running a match

```
export EXAMPLE_BOT=target/riscv32imac-unknown-none-elf/debug/ff-example-bot
cargo run -p ff_local -- $EXAMPLE_BOT $EXAMPLE_BOT
```

(The `export` here is just to make the tabove more readable. You can just type
the paths out).

Adding `RUST_LOG=debug` will log some useful info about what is happening
in the VM and the Ferris Fencing runtime.


### Building your own bot

Either edit `example_bot` in place or copy it elsewhere as a template to work
off of.


### The RISC-V gcc toolchain

Having a RISC-V gcc toolchain may be useful for debugging, assembly,
dissassamebly, and writing C. The following commands will build and install them
to `$HOME/riscv-gcc`.

```
git clone --recursive https://github.com/riscv/riscv-gnu-toolchain
cd riscv-gnu-toolchain
mkdir build && cd build
../configure --prefix=$HOME/riscv-gcc --with-arch=rv32imac --with-abi=ilp32
make install
```

For example, to decompile the example bot:

```
 ~/riscv-gcc/bin/riscv32-unknown-elf-objdump -d target/riscv32imac-unknown-none-elf/debug/ff-example-bot
```


## Roadmap

Here's a vague description of the MVP:

- Players can upload bots via the `ff_upload` command.
- Player bots consist of an `elf` exe, a single-grapheme name (emoji
  encouraged), and a 128-bit random identifier.
- The website is static but contains a live.html frame that contains all the
  dynamic logic. This frame can be embedded elsewhere as needed.
- On load, live.html requests a random match; the server generates
  it and responds; live.html interprets the results by moving
  Ferris, energy bars, move indicators, and the scoreboard.
- After a match is complete, live.html requests another.
- The number of bots is capped to prevent abuse, and are "garbage collected" in
  FIFO order.
- The game rules are improved to be interesting.

There is extensive potential beyond the MVP, but this first.


## Contributions

Contributions for bug fixes, and toward the MVP, welcome.


## License

Apache-2.0/MIT/BSL-1.0/CC-0
