# orbclient
The Orbital Client Library. Compatible with Redox and SDL2.

[![Build status](https://gitlab.redox-os.org/redox-os/orbclient/badges/master/pipeline.svg)](https://gitlab.redox-os.org/redox-os/orbclient/pipelines)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)
[![crates.io](http://meritbadge.herokuapp.com/orbclient)](https://crates.io/crates/orbclient)
[![docs.rs](https://docs.rs/orbclient/badge.svg)](https://docs.rs/orbclient)

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
* Other problem? Do not hesitate to create a new issue!
