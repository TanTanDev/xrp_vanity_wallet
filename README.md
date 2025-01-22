# Xrp Vanity Wallet generator
written in rust, utilizing xrpl-rust library for wallet creation

## how to use 
add words you want to appear in your addresses in the file: names.txt
run, and wait for wallets to be generated and outputed as files in output/*.txt
edit source to customize filtering needs.
currently word capitalization doesn't matter, and only where the word is at start or end of address are saved.

## CAUTION
use at your own risk, wallet creation utilize the xrpl-rust crate, and from what I've seen it appears to be safe(but I'm no crypto expert). 
Always check the source before using.

## License
xrp_vanity_wallet is free and open source! All code in this repository is dual-licensed under either:

* MIT License ([LICENSE-MIT](docs/LICENSE-MIT) or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))
* Apache License, Version 2.0 ([LICENSE-APACHE](docs/LICENSE-APACHE) or [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0))

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
