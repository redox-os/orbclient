# orbclient
The Orbital Client Library. Compatible with Redox and SDL2.

[![Build status](https://gitlab.redox-os.org/redox-os/orbclient/badges/master/pipeline.svg)](https://gitlab.redox-os.org/redox-os/orbclient/pipelines)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)
[![crates.io](https://img.shields.io/crates/v/orbclient)](https://crates.io/crates/orbclient)
[![docs.rs](https://docs.rs/orbclient/badge.svg)](https://docs.rs/orbclient)

## Features
The "serde" feature can be used to include code for `Color` deserialization using the `serde` crate (which is an 
optional dependency). This is not enabled by default. To enable, either build using the `--features "serde"` command
line option, or use `features = ["serde"]` in your crate, where it declares a dependency on orbclient.

## Platform notes 

### Web

* Rust standard toolchain `rustup`, `rustc`, `cargo` ([install](https://www.rust-lang.org/tools/install))
* Rust web assembly toolchain `wasm-pack` ([install](https://rustwasm.github.io/wasm-pack/installer/))
* JavaScript package manager npm ([install](https://www.npmjs.com/get-npm))
* Run simple example
  * Navigate to `examples/simple` director
  * Run `npm install`
  * Run `npm run serve`

### Troubleshooting

* Make sure that you work with the current ```nightly``` version of Rust
  * To make sure of that, please use [rustup](https://github.com/rust-lang-nursery/rustup.rs)
  * Don't forget to override your work directory with ```rustup override set nightly```
  * Don't forget to update the ```nightly``` version of Rust with ```rustup update nightly```
* SDL2 should be automatically with orbclient if you have trouble try to install it ```libsdl2-dev``` manually   
  * For example, with Ubuntu, please to type ```sudo apt-get install libsdl2-dev``` in your console
* On fedora please type ```sudo dnf install SDL2-devel SDL2-static``` in your console before building.
  * if during building, this message comes up ```could not find native static library `SDL2main`, perhaps an -L flag is missing?```.
   Providing the path to the static library might help. You can provide this path via ```RUSTFLAGS='-L <path-to-folder-with-libSDL2.a>' cargo b ...```.
   At the moment of writing, the SDL2 library is stored under **/usr/lib64** on fedora. In this case you would type ```RUSTFLAGS='-L /usr/lib64' cargo r --example simple``` 
   to start the simple example.
* Other problem? Do not hesitate to create a new issue!
