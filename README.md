# sr25519 for embedded device

## Why embedded device

Hardware wallet, HSM or IOT modules may need run sr25519 to make signatures or derive keys if they serve Polkadot or para chains on Substrate

### features

Embedded devices always have small storage size due to the cost of chip, most of them have less then 2MBytes for all data and code. The same situation for RAM, less them 512KBytes. And the frequency is less then 512Mhz usually. And most of all, they may be none-os devices. It means there is no multi-task, thead, memory management or error handler at all.

### Our target
To build a C wrapper for [w3f/schnorrkel](https://github.com/w3f/schnorrkel) and compile it to static lib without std. So that embedded developers could use this static lib to work with other C code. Comparing the std version C wrapper [Warchant](https://github.com/Warchant/sr25519-crust) the embedded version has the following troubles to deal with:

1. [no std] means no libc types and ptr to transfer data bewteen RUST and C. So we just use basic types and boxed struct to return data to C level.
2. No default memory allocator. So if we want to keep data in heap which is must to do when C calling RUST, we need build an allocator by ourselves.
3. Handle RNG right since rand_core, rand_chacha or other rng libs have many issues for most of the platform mentioned above. Attach_rng in schnorrkel is a good choice so we can give external RNG to the lib.
4. Reduce code size and memory usage.

## Set the toolchain

Please ref the RUST embedded book to get infomation if the document missed.
[embedded book](https://rust-embedded.github.io/book/intro/install.html)

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

### Config, build and run

To build the project. If you don't use the config file, the it will be

```shell
cargo build --release --target thumbv7m-none-eabi
```

## How to work with exsiting project

C is widely used language for embedded. So, we should find a way to let RUST code with C code. The solution we provide is to compile static library in RUST and then link it with arm-gcc to bin file.

 Now we have the libsr.a in target folder. You could copy it to your project directory. Please note that you MUST choose arm-gcc to link it as the first step because ARMCC may generate some errors.

### C wrapper

wrapper.rs adds a layer which could access by C. Note that libc could not be used for cortex so that we just use the basic types in RUST. To return to C return, a struct called sr_data is defined and boxed in heap. We simply return all data in this struct to avoid more type issues. The heap must be init by sr_init and a global var must be defined in C to give the start address of your heap.

### RNG

All the method on none-os platform may break down so to handle the random number generate task could be the embedded developer's work. To attach the random into sr25519, we ref attach_rng in schnorrkel.

```
struct ExternalRng;
    impl RngCore for ExternalRng {
        fn next_u32(&mut self) -> u32 {  panic!()  }
        fn next_u64(&mut self) -> u64 {  panic!()  }
        fn fill_bytes(&mut self, dest: &mut [u8]) {
            //zero here
            //or you could get your rng in
            for i in dest.iter_mut() {  *i = 0;  }
        }
        fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), ::rand_core::Error> {
            self.fill_bytes(dest);
            Ok(())
        }
    }
    impl CryptoRng for ExternalRng {}

    let signature: Signature = keypair.sign(attach_rng(context.bytes(&message_bytes[..]), ExternalRng));
```
