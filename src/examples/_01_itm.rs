//! Sends "Hello, world!" through the first ITM stimulus port
//!
//! To receive the message on the host you'll have to do three things:
//!
//! You'll need to uncomment lines 8 and 15 of the `.gdbinit` file.
//!
//! You'll also need to connect the SWO pin of the on-board SWD programmer to pin PB3 as shown
//! [here](https://japaric.github.io/discovery/06-hello-world/README.html).
//!
//! Finally, you'll need to run `itmdump itm.fifo` (mind the file paths) to receive the message.
//! Read the documentation of the [`itm`] crate, which provides the `itmdump` tool,  for details.
//!
//! [`itm`]: https://docs.rs/itm/0.2.0/itm/
//!
//! ```
//! #![deny(unsafe_code)]
//! #![deny(warnings)]
//! #![no_std]
//! 
//! #[macro_use]
//! extern crate cortex_m;
//! extern crate f3;
//! 
//! fn main() {
//!     let p = cortex_m::Peripherals::take().unwrap();
//!     let mut itm = p.ITM;
//! 
//!     iprintln!(&mut itm.stim[0], "Hello, world!");
//! }
//! ```
// Auto-generated. Do not modify.
