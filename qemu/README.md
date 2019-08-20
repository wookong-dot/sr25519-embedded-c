# Build sr25519 for CortexMx

## Embedded

Schnorrkel may be used in embedded system such as hardware wallet or HSM. To build the rust code to a static library and then link with other code which is commonly C seems a good way to go. And as far as we know, the embedded may have hardware random source so it is not necessary to generate it which bring std depending issues for embedded compile in this crate. So we bring the ```ext_rng``` features for the embedded compile as well we project based on QEMU  to test it.

## Set the toolchain

Please ref the RUST embedded book to get infomation if the document missed.
[embedded book](https://rust-embedded.github.io/book/intro/install.html)

### install QEMU

QEMU is QEMU is a generic and open source machine emulator and virtualizer. It could be used to simulate cortexMx serial mcu on pc.

```shell
brew install qemu
```

Here is a ref link to know QEMU more:
[qemu guide](https://rust-embedded.github.io/book/start/qemu.html)

### Rust target and llvm

```shell
rustup target add thumbv7m-none-eabi
rustup component add llvm-tools-preview
```

Here is the core list if you need.

```shell
thumbv6m-none-eabi //for the Cortex-M0 and Cortex-M1 processors
thzumbv7m-none-eabi //for the Cortex-M3 processor
thumbv7em-none-eabi //for the Cortex-M4 and Cortex-M7 processors
thumbv7em-none-eabihf //for the Cortex-M4F and Cortex-M7F processors
```

### Before build

Please do close the 'lib' sections in toml file of schnorrkel so we can make it crate. And close the default feature in 'features' which is confused me because we have use embedded feature from this crate.

### Config, build and run

We already provide config file in ./cargo/config. Please note the default target is thumbv7m-none-eabi. So you can use

```shell
cargo build --release
cargo run --release
```

to build and run the project. If you don't use the config file, the it will be

```shell
cargo build --release --target thumbv7m-none-eabi
cargo run --release --target thumbv7m-none-eabi
```

### Inspecting

Now we have a non-native ELF binary in target/thumbv7m-none-eabi/release/cortexm. We can inspect it using cargo-binutils.

```shell
cargo readobj --bin cortexm -- -file-headers
cargo size --bin cortexm --release -- -A
```

Please do keep --release tag because the LM3S6965 provided by QEMU only has 256KB flash. The bin size of debug mode is over the limited.

### Issues

We met a rustc version issue, please ref the link if have "'.ARM.exidx'" link errors.
[issue](https://github.com/rust-lang/rust/issues/62781)

## How to work with exsiting project

C is widely used language for embedded. So, we should find a way to let RUST code with C code. The solution we provide is to compile static library in RUST and then link it with arm-gcc to bin file. 

### compile

To compile the static lib, you should:
open the 'lib' section in schnorrkel's toml

```toml
[lib]
name = "sr"
path = "src/lib.rs"
crate-type = ["staticlib"]
```

and close [dev-dependencies]

```toml
# [dev-dependencies]
# hex = "^0.3"
# sha2 = "^0.8"
# bincode = "^0.9"
# criterion = "0.2"
```

Then use  

```shell
cargo build --release --target thumbv7m-none-eabi --no-default-features --features "embedded"
```

to compile it. Now we have the libsr.a in target folder. You could copy it to your project directory. Please note that you MUST choose arm-gcc to link it as the first step because ARMCC may generate some errors.

### C wrapper

wrapper.rs adds a layer which could access by C. Note that libc could not be used for cortex so that we just use the basic types in RUST. To return to C return, a struct called sr_data is defined and boxed in heap. We simply return all data in this struct to avoid more type issues. The heap must be init by sr_init and a global var must be defined in C to give the start address of your heap.
